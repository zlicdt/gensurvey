//
//  main.rs
//  gensurvey-server
//
//  Created by zlicdt on 2025/8/13.
//  Copyright (c) 2025 zlicdt. All rights reserved.
//
//  This file is part of gensurvey.
//
//  gensurvey is free software: you can redistribute it and/or modify
//  it under the terms of the GNU Affero General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  gensurvey is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU Affero General Public License for more details.
//
//  You should have received a copy of the GNU Affero General Public License
//  along with gensurvey. If not, see <https://www.gnu.org/licenses/>.
//
use std::{net::SocketAddr};
use axum::{routing::{get, post}, Router, Json, extract::State, http::StatusCode, response::{IntoResponse, Response}};
use serde::{Deserialize, Serialize};
use clap::Parser;
use tracing::{info, error};
use sqlx::Row; // for dynamic row access
use tower_http::cors::{CorsLayer, Any};
use std::path::Path;
use sqlx::sqlite::SqliteConnectOptions;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(name="gensurvey-server", version, about="Simple survey submission receiver")] 
struct Args {
    /// Listen port (default 11451)
    #[arg(short = 'p', long = "port", default_value_t = 11451)]
    port: u16,

    /// Admin mode
    #[arg(short = 'A', long = "admin_mode", default_value_t = false)]
    admin_mode: bool,

    /// Path to sqlite database file (default /usr/lib/gensurvey/gensurvey.db)
    #[arg(long = "db-path", value_name = "PATH", default_value = "/usr/lib/gensurvey/gensurvey.db")]
    db_path: String,
}

#[derive(Clone)]
struct AppState {
    pool: sqlx::SqlitePool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmissionPayload(serde_json::Value);

#[derive(Debug, Serialize, Clone, sqlx::FromRow)]
pub struct SubmissionRecord {
    id: i64,
    received_at: String,
    data: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct SubmitResponse { id: u64, status: &'static str }

#[derive(Debug, Serialize)]
struct ErrorResponse { error: String }

// Custom error type for better error handling
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("Request error: {}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { 
                error: "Internal server error".to_string() 
            })
        ).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self(err.into())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self(err.into())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let args = Args::parse();

    // Ensure parent directory exists (ignore error if lacking permission; will surface in connect)
    if let Some(parent) = Path::new(&args.db_path).parent() {
        if !parent.exists() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                error!(dir=?parent, error=%e, "Failed to create db directory");
            }
        }
    }

    // Try multiple forms to build connect options. Primary: treat provided path as plain filename.
    let connect_opts = SqliteConnectOptions::from_str(&args.db_path)
        .or_else(|_| SqliteConnectOptions::from_str(&format!("sqlite://{}", &args.db_path)))
        .or_else(|_| SqliteConnectOptions::from_str(&format!("sqlite:{}", &args.db_path)))
        .map(|o| o.create_if_missing(true))
        .map_err(|e| anyhow::anyhow!("Invalid sqlite path '{}': {}", &args.db_path, e))?;

    let pool = sqlx::SqlitePool::connect_with(connect_opts).await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;
    
    // migrate table
    sqlx::query(r#"CREATE TABLE IF NOT EXISTS submissions(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        received_at TEXT NOT NULL,
        data TEXT NOT NULL
    )"#).execute(&pool).await
        .map_err(|e| anyhow::anyhow!("Failed to create table: {}", e))?;
    let state = AppState { pool };

    let app = if args.admin_mode {
        Router::new()
            .route("/health", get(|| async { "OK" }))
            .route("/submissions", get(list_submissions))
            .route("/submit", post(receive_submission))
            .with_state(state.clone())
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any)
            )
    } else {
        // Normal mode, user have no access to /submissions
        Router::new()
            .route("/health", get(|| async { "OK" }))
            .route("/submit", post(receive_submission))
            .with_state(state.clone())
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any)
            )
    };

    let addr = SocketAddr::from(([0,0,0,0], args.port));
    info!(%addr, "Server listening");
    
    let listener = tokio::net::TcpListener::bind(addr).await
        .map_err(|e| anyhow::anyhow!("Failed to bind to {}: {}", addr, e))?;
    
    axum::serve(listener, app).await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;
    
    Ok(())
}

async fn receive_submission(
    State(state): State<AppState>, 
    Json(payload): Json<SubmissionPayload>
) -> Result<Json<SubmitResponse>, AppError> {
    let ts = chrono::Utc::now().to_rfc3339();
    let data_str = serde_json::to_string(&payload.0)?;
    
    let rec_id = sqlx::query("INSERT INTO submissions(received_at,data) VALUES(?,?)")
        .bind(ts)
        .bind(data_str)
        .execute(&state.pool)
        .await?
        .last_insert_rowid();
    
    Ok(Json(SubmitResponse { 
        id: rec_id as u64, 
        status: "stored" 
    }))
}

async fn list_submissions(
    State(state): State<AppState>
) -> Result<Json<Vec<SubmissionRecord>>, AppError> {
    let rows = sqlx::query("SELECT id, received_at, data FROM submissions ORDER BY id DESC")
        .fetch_all(&state.pool)
        .await?;
    
    let mapped: Vec<SubmissionRecord> = rows.into_iter().map(|row| {
        let id: i64 = row.get("id");
        let received_at: String = row.get("received_at");
        let data_str: String = row.get("data");
        let data: serde_json::Value = serde_json::from_str(&data_str)
            .unwrap_or(serde_json::Value::Null);
        SubmissionRecord { id, received_at, data }
    }).collect();
    
    Ok(Json(mapped))
}
