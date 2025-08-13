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
use axum::{routing::{get, post}, Router, Json, extract::State};
use serde::{Deserialize, Serialize};
use clap::Parser;
use tracing::{info, error};
use sqlx::Row; // for dynamic row access
use tower_http::cors::{CorsLayer, Any};

#[derive(Parser, Debug)]
#[command(name="gensurvey-server", version, about="Simple survey submission receiver")] 
struct Args {
    /// Listen port (default 11451)
    #[arg(short = 'p', long = "port", default_value_t = 11451)]
    port: u16,

    /// Admin mode
    #[arg(short = 'A', long = "admin_mode", default_value_t = false)]
    admin_mode: bool,

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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let args = Args::parse();
    let pool = sqlx::SqlitePool::connect("sqlite:file:gensurvey.db?mode=rwc").await.expect("connect sqlite");
    // migrate table
    sqlx::query(r#"CREATE TABLE IF NOT EXISTS submissions(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        received_at TEXT NOT NULL,
        data TEXT NOT NULL
    )"#).execute(&pool).await.expect("create table");
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
    if let Err(e) = axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await {
        error!(error=%e, "Server error");
    }
}

async fn receive_submission(State(state): State<AppState>, Json(payload): Json<SubmissionPayload>) -> Json<SubmitResponse> {
    let ts = chrono::Utc::now().to_rfc3339();
    let data_str = serde_json::to_string(&payload.0).unwrap();
    let rec_id = sqlx::query("INSERT INTO submissions(received_at,data) VALUES(?,?)")
        .bind(ts)
        .bind(data_str)
        .execute(&state.pool)
        .await
        .unwrap()
        .last_insert_rowid();
    Json(SubmitResponse { id: rec_id as u64, status: "stored" })
}

async fn list_submissions(State(state): State<AppState>) -> Json<Vec<SubmissionRecord>> {
    let rows = sqlx::query("SELECT id, received_at, data FROM submissions ORDER BY id DESC")
        .fetch_all(&state.pool)
        .await
        .unwrap();
    let mapped: Vec<SubmissionRecord> = rows.into_iter().map(|row| {
        let id: i64 = row.get("id");
        let received_at: String = row.get("received_at");
        let data_str: String = row.get("data");
        let data: serde_json::Value = serde_json::from_str(&data_str).unwrap_or(serde_json::Value::Null);
        SubmissionRecord { id, received_at, data }
    }).collect();
    Json(mapped)
}
