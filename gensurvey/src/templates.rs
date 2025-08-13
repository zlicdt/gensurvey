// HTML templates extracted for maintainability.
// Placeholders use the form {name} and are replaced by simple string substitution (no escaping done here).
// Make sure to escape dynamic values before passing them.

pub const PAGE_SHELL: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8"/>
<meta name="viewport" content="width=device-width,initial-scale=1"/>
<title>{title}</title>
<script>window.tailwind=window.tailwind||{};</script>
<script src="https://cdn.tailwindcss.com" referrerpolicy="no-referrer"></script>
<script>
window.addEventListener('load',()=>{try{var t=document.createElement('span');t.className='hidden';document.body.appendChild(t);var hasTailwind=window.getComputedStyle(t).display==='none';t.remove();if(!hasTailwind){const s=document.createElement('style');s.textContent='body{font-family:system-ui,sans-serif;margin:1.5rem;background:#f9fafb;color:#111827}fieldset{margin-bottom:1.25rem;padding:1rem;border:1px solid #e5e7eb;border-radius:.75rem;background:#fff}button{background:#2563eb;color:#fff;border:none;padding:.6rem 1.1rem;border-radius:.5rem;cursor:pointer}button:hover{background:#1d4ed8}.hidden{display:none}';document.head.appendChild(s);}}catch(e){/* ignore */}});
</script>
</head>
<body class="bg-gray-50 text-gray-900 antialiased">
<main class="max-w-3xl mx-auto p-6 md:p-10">
<header class="mb-10"><h1 class="text-3xl font-bold tracking-tight mb-3">{title}</h1><p class="text-gray-600 text-lg leading-relaxed">{description}</p></header>
<form id="survey-form" class="space-y-8" data-endpoint="{endpoint}">
{content}
<div><button type="submit" class="inline-flex items-center gap-2 px-6 py-2 rounded-md bg-blue-600 text-white font-medium shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-600 transition">Submit<span class="sr-only"> survey</span></button></div>
</form>
<div id="result" class="text-sm text-gray-600 mt-6"></div>
</main>
<script src="script.js"></script>
</body>
</html>
"#;

pub const QUESTION_FIELDSET: &str = r#"<fieldset id="{id}" class="bg-white rounded-xl border border-gray-200 shadow-sm p-6 md:p-8 space-y-5" aria-labelledby="{id}-label">
<div id="{id}-label" class="text-lg font-semibold flex items-start gap-2 mb-2">{text}{required_mark}</div>
<div class="options space-y-2" {options_attrs}>
{controls}
</div>
{subs}
</fieldset>
"#;

pub const SUB_QUESTION_BLOCK: &str = r#"<div class="subq border-l-4 border-gray-200 pl-4 mt-4 space-y-2{sub_hidden}" id="sub-{id}">
<div class="text-sm font-medium text-gray-700 flex items-center gap-2">{text}{required_mark}</div>
{input}{condition_marker}
</div>
"#;

pub fn fill(mut tpl: String, replacements: &[(&str, &str)]) -> String {
    for (k,v) in replacements {
        let ph = format!("{{{}}}", k);
        tpl = tpl.replace(&ph, v);
    }
    tpl
}
