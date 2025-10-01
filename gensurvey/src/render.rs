//
//  render.rs
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
use crate::model::{Question, QuestionType, SubQuestion, Survey};
use crate::templates::{PAGE_SHELL, QUESTION_FIELDSET, SUB_QUESTION_BLOCK, fill};

pub fn render_full_html(survey: &Survey) -> String {
  let mut content = String::new();
  for q in &survey.questions { content.push_str(&render_question(q)); }
  // Derive submission endpoint: if user supplies a base like http://host:port, append /submit.
  let endpoint = survey
    .gensurvey_server
    .as_ref()
    .map(|raw| {
      let mut url = raw.trim().to_string();
      if !url.is_empty() && !url.contains('?') {
        // remove trailing slash (but keep single root slash after scheme)
        while url.ends_with('/') && !url.ends_with("//") { url.pop(); }
        if !url.ends_with("/submit") { url.push_str("/submit"); }
      }
      escape(&url)
    })
    .unwrap_or_else(|| "".to_string());
  fill(PAGE_SHELL.to_string(), &[
    ("title", &escape(&survey.title)),
    ("description", &escape(&survey.description)),
    ("content", &content),
    ("endpoint", &endpoint),
  ("script", ""),
  ])
}

fn render_question(q: &Question) -> String {
  let mut controls = String::new();

    match q.kind {
  QuestionType::SingleChoice => {
      for (idx,opt) in q.options.iter().enumerate() {
    controls.push_str("<label class=\"group flex items-center gap-3 p-4 rounded-xl border-2 border-gray-200 hover:border-indigo-400 hover:bg-indigo-50 cursor-pointer transition-all duration-200\"><input class=\"h-5 w-5 text-indigo-600 focus:ring-2 focus:ring-indigo-500 border-gray-300 cursor-pointer\" type=\"radio\" name=\"");
        controls.push_str(&escape(&q.id));
        controls.push_str("\" value=\"");
        controls.push_str(&escape(&opt.value));
        controls.push_str("\"");
        if q.required && idx==0 { /* placeholder */ }
        controls.push_str("/> <span class=\"text-base font-medium text-gray-700 group-hover:text-gray-900\">");
        controls.push_str(&escape(&opt.label));
        controls.push_str("</span></label>");
      }
    }
    QuestionType::MultipleChoice => {
      for opt in &q.options { 
        controls.push_str("<label class=\"group flex items-center gap-3 p-4 rounded-xl border-2 border-gray-200 hover:border-indigo-400 hover:bg-indigo-50 cursor-pointer transition-all duration-200\"><input class=\"h-5 w-5 text-indigo-600 focus:ring-2 focus:ring-indigo-500 rounded border-gray-300 cursor-pointer\" type=\"checkbox\" name=\"");
        controls.push_str(&escape(&q.id));
        controls.push_str("\" value=\"");
        controls.push_str(&escape(&opt.value));
        controls.push_str("\"/> <span class=\"text-base font-medium text-gray-700 group-hover:text-gray-900\">");
        controls.push_str(&escape(&opt.label));
        controls.push_str("</span></label>");
      }
    }
    QuestionType::Text => {
      controls.push_str("<input type=\"text\" name=\"");
      controls.push_str(&escape(&q.id));
      controls.push_str("\" class=\"w-full rounded-xl border-2 border-gray-200 focus:border-indigo-500 focus:ring-4 focus:ring-indigo-100 text-gray-900 placeholder-gray-400 px-4 py-3 transition-all duration-200\" placeholder=\"Enter your answer...\" ");
      if q.required { controls.push_str("required "); }
      controls.push_str("/>\n");
    }
    }
  let mut subs = String::new();
  for sub in &q.sub_questions { subs.push_str(&render_sub_question(q, sub)); }
  let required_mark = if q.required { "<span class=\"text-red-500 font-normal\" title=\"Required\">*</span>" } else { "" };
  let options_attrs = match q.kind { 
    QuestionType::MultipleChoice => if q.required { "data-required-group=true" } else { "" },
    _ => "",
  };
  fill(QUESTION_FIELDSET.to_string(), &[
    ("id", &escape(&q.id)),
    ("text", &escape(&q.text)),
    ("required_mark", required_mark),
    ("controls", &controls),
    ("subs", &subs),
    ("options_attrs", options_attrs),
  ])
}

fn render_sub_question(_parent: &Question, sub: &SubQuestion) -> String {
  let required_mark = if sub.required { "<span class=\"text-red-500\" title=\"Required\">*</span>" } else { "" };
  let mut input = String::new();
  match sub.kind {
    QuestionType::Text => {
      input.push_str("<input type=\"text\" name=\"");
      input.push_str(&escape(&sub.id));
  input.push_str("\" class=\"w-full rounded-lg border-2 border-indigo-200 focus:border-indigo-400 focus:ring-4 focus:ring-indigo-100 text-gray-900 placeholder-gray-400 px-4 py-2.5 bg-white transition-all duration-200\" placeholder=\"Enter your answer...\" ");
      if sub.required { input.push_str("required "); }
      input.push_str("/>");
    }
    QuestionType::SingleChoice | QuestionType::MultipleChoice => {
      input.push_str("<input type=\"text\" name=\"");
      input.push_str(&escape(&sub.id));
  input.push_str("\" class=\"w-full rounded-lg border-2 border-indigo-200 focus:border-indigo-400 focus:ring-4 focus:ring-indigo-100 text-gray-900 px-4 py-2.5 bg-white transition-all duration-200\" />");
    }
  }
  let mut condition_marker = String::new();
  if let Some(cond) = &sub.when_display {
    let mut display_parent = cond.condition.clone();
    if _parent.options.iter().any(|o| o.value == cond.value) {
      display_parent = _parent.id.clone();
    }
    condition_marker.push_str("<input type=\"hidden\" data-display-parent=\"");
    condition_marker.push_str(&escape(&display_parent));
    condition_marker.push_str("\" data-display-value=\"");
    condition_marker.push_str(&escape(&cond.value));
    condition_marker.push_str("\"/>");
  }
  let hidden = if sub.hide { " hidden" } else { "" };
  fill(SUB_QUESTION_BLOCK.to_string(), &[
    ("id", &escape(&sub.id)),
    ("text", &escape(&sub.text)),
    ("required_mark", required_mark),
    ("input", &input),
    ("condition_marker", &condition_marker),
    ("sub_hidden", hidden),
  ])
}

fn escape(s: &str) -> String { htmlescape::encode_minimal(s) }
