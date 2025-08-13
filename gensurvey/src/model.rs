//
//  model.rs
//  gensurvey
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
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    SingleChoice,
    MultipleChoice,
    Text,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DisplayCondition {
    pub condition: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionItem {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: String,
    pub text: String,
    #[serde(rename = "type")]
    pub kind: QuestionType,
    #[serde(default)]
    pub options: Vec<OptionItem>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub sub_questions: Vec<SubQuestion>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubQuestion {
    pub id: String,
    pub text: String,
    #[serde(rename = "type")]
    pub kind: QuestionType,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub hide: bool,
    #[serde(default)]
    pub when_display: Option<DisplayCondition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Survey {
    pub title: String,
    pub description: String,
    pub gensurvey_server: Option<String>,
    pub questions: Vec<Question>,
}

impl Survey {
    pub fn from_reader<R: std::io::Read>(mut r: R) -> anyhow::Result<Self> {
        let mut buf = String::new();
        // use std::io::Read;
        r.read_to_string(&mut buf)?;
        // Allow // style comments by stripping them.
        let cleaned = strip_jsonc(&buf);
        let survey: Survey = match serde_json::from_str(&cleaned) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to parse JSON. Cleaned content:\n{}", cleaned);
                return Err(e.into());
            }
        };
        Ok(survey)
    }
}

fn strip_jsonc(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_str = false;
    let mut escape = false;
    while let Some(c) = chars.next() {
        if in_str {
            if escape { // current char is escaped
                out.push(c);
                escape = false;
                continue;
            }
            match c {
                '\\' => { out.push(c); escape = true; },
                '"' => { in_str = false; out.push(c); },
                _ => out.push(c),
            }
            continue;
        } else {
            if c == '"' { in_str = true; out.push(c); continue; }
            if c == '/' {
                if let Some('/') = chars.peek() { // start of // comment
                    // consume second /
                    chars.next();
                    // skip until newline
                    while let Some(&nc) = chars.peek() { if nc == '\n' { break; } chars.next(); }
                    continue; // discard comment
                }
            }
            out.push(c);
        }
    }
    out
}
