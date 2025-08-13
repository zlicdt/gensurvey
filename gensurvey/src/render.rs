use crate::model::{Question, QuestionType, SubQuestion, Survey};
use crate::templates::{PAGE_SHELL, QUESTION_FIELDSET, SUB_QUESTION_BLOCK, fill};

pub fn render_full_html(survey: &Survey) -> String {
  let mut content = String::new();
  for q in &survey.questions { content.push_str(&render_question(q)); }
  let endpoint = survey
    .gensurvey_server
    .as_ref()
    .map(|e| escape(e))
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
    controls.push_str("<label class=\"flex items-center gap-2 text-gray-800\"><input class=\"h-4 w-4 text-blue-600 focus:ring-blue-600 border-gray-300\" type=\"radio\" name=\"");
        controls.push_str(&escape(&q.id));
        controls.push_str("\" value=\"");
        controls.push_str(&escape(&opt.value));
        controls.push_str("\"");
        if q.required && idx==0 { /* placeholder */ }
        controls.push_str("/> <span>");
        controls.push_str(&escape(&opt.label));
        controls.push_str("</span></label>");
      }
    }
    QuestionType::MultipleChoice => {
      for opt in &q.options { 
        controls.push_str("<label class=\"flex items-center gap-2 text-gray-800\"><input class=\"h-4 w-4 text-blue-600 focus:ring-blue-600 rounded border-gray-300\" type=\"checkbox\" name=\"");
        controls.push_str(&escape(&q.id));
        controls.push_str("\" value=\"");
        controls.push_str(&escape(&opt.value));
        controls.push_str("\"/> <span>");
        controls.push_str(&escape(&opt.label));
        controls.push_str("</span></label>");
      }
    }
    QuestionType::Text => {
      controls.push_str("<input type=\"text\" name=\"");
      controls.push_str(&escape(&q.id));
      controls.push_str("\" class=\"w-full max-w-md rounded-md border-gray-300 focus:border-blue-600 focus:ring-blue-600 text-gray-900 placeholder-gray-400\" placeholder=\"Enter text...\" ");
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
  input.push_str("\" class=\"w-full max-w-sm rounded-md border-gray-300 focus:border-blue-600 focus:ring-blue-600 text-gray-900 placeholder-gray-400\" placeholder=\"Enter text...\" ");
      if sub.required { input.push_str("required "); }
      input.push_str("/>");
    }
    QuestionType::SingleChoice | QuestionType::MultipleChoice => {
      input.push_str("<input type=\"text\" name=\"");
      input.push_str(&escape(&sub.id));
  input.push_str("\" class=\"w-full max-w-sm rounded-md border-gray-300 focus:border-blue-600 focus:ring-blue-600 text-gray-900\" />");
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
