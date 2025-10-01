//
//  templates.rs
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
<style>
@keyframes fadeIn { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
@keyframes slideIn { from { opacity: 0; transform: translateX(-20px); } to { opacity: 1; transform: translateX(0); } }
@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: .8; } }
@keyframes success { 0% { transform: scale(0.95); } 50% { transform: scale(1.05); } 100% { transform: scale(1); } }
body { animation: fadeIn 0.6s ease-out; }
.question-card { animation: slideIn 0.5s ease-out backwards; }
.question-card:nth-child(1) { animation-delay: 0.1s; }
.question-card:nth-child(2) { animation-delay: 0.2s; }
.question-card:nth-child(3) { animation-delay: 0.3s; }
.question-card:nth-child(4) { animation-delay: 0.4s; }
.question-card:nth-child(5) { animation-delay: 0.5s; }
input[type="radio"]:checked, input[type="checkbox"]:checked { animation: success 0.3s ease; }
.gradient-bg { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); }
.glass-card { background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); }
</style>
<script>
window.addEventListener('load',()=>{try{var t=document.createElement('span');t.className='hidden';document.body.appendChild(t);var hasTailwind=window.getComputedStyle(t).display==='none';t.remove();if(!hasTailwind){const s=document.createElement('style');s.textContent='body{font-family:system-ui,sans-serif;padding:1.5rem;background:linear-gradient(135deg,#667eea 0%,#764ba2 100%);color:#111827;min-height:100vh}fieldset{margin-bottom:1.25rem;padding:1.5rem;border:none;border-radius:1rem;background:rgba(255,255,255,0.95);box-shadow:0 4px 6px rgba(0,0,0,0.1)}button{background:#667eea;color:#fff;border:none;padding:.75rem 1.5rem;border-radius:.75rem;cursor:pointer;font-weight:600;box-shadow:0 4px 6px rgba(0,0,0,0.1);transition:all 0.3s}button:hover{background:#5568d3;transform:translateY(-2px);box-shadow:0 6px 12px rgba(0,0,0,0.15)}.hidden{display:none}label{transition:all 0.2s}label:hover{transform:translateX(4px)}input[type=text]{transition:all 0.2s}input[type=text]:focus{transform:scale(1.01)}';document.head.appendChild(s);}}catch(e){/* ignore */}});
</script>
</head>
<body class="min-h-screen bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 text-gray-900 antialiased">
<div class="absolute inset-0 bg-[url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNjAiIGhlaWdodD0iNjAiIHZpZXdCb3g9IjAgMCA2MCA2MCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48ZyBmaWxsPSJub25lIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiPjxnIGZpbGw9IiNmZmZmZmYiIGZpbGwtb3BhY2l0eT0iMC4wNSI+PHBhdGggZD0iTTM2IDM0djItaDJ2LTJoLTJ6bTAtNHYyaDJ2LTJoLTJ6bTAtNHYyaDJ2LTJoLTJ6bTAtNHYyaDJ2LTJoLTJ6bTAtNHYyaDJ2LTJoLTJ6bTAtNHYyaDJ2LTJoLTJ6bTAtNHYyaDJ2LTJoLTJ6bTAtNHYyaDJ2LTJoLTJ6bTAtNHYyaDJ2LTJoLTJ6bS00IDB2Mmgydi0yaC0yem0tNCAyaDJ2LTJoLTJ2MnptLTQgMGgydi0yaC0ydjJ6bS00IDBoMnYtMmgtMnYyem0tNCAwaDJ2LTJoLTJ2MnptLTQgMGgydi0yaC0ydjJ6bS00IDBoMnYtMmgtMnYyem0tNCAwaDJ2LTJoLTJ2MnptLTQgMGgydi0yaC0ydjJ6bTAgNHYtMmgtMnYyaDJ6bTAgNHYtMmgtMnYyaDJ6bTAgNHYtMmgtMnYyaDJ6bTAgNHYtMmgtMnYyaDJ6bTAgNHYtMmgtMnYyaDJ6bTAgNHYtMmgtMnYyaDJ6bTAgNHYtMmgtMnYyaDJ6bTAgNHYtMmgtMnYyaDJ6Ii8+PC9nPjwvZz48L3N2Zz4=')] opacity-30"></div>
<main class="relative max-w-4xl mx-auto p-4 md:p-8 lg:p-12">
<div class="glass-card rounded-3xl shadow-2xl p-8 md:p-12 mb-8">
<header class="mb-12 text-center">
<h1 class="text-4xl md:text-5xl font-bold tracking-tight mb-4 bg-gradient-to-r from-indigo-600 to-purple-600 bg-clip-text text-transparent">{title}</h1>
<p class="text-gray-600 text-lg md:text-xl leading-relaxed max-w-2xl mx-auto">{description}</p>
<div class="mt-6 h-1 w-24 mx-auto bg-gradient-to-r from-indigo-500 to-purple-500 rounded-full"></div>
</header>
<form id="survey-form" class="space-y-6" data-endpoint="{endpoint}">
{content}
<div class="pt-6 flex justify-center">
<button type="submit" class="group relative inline-flex items-center gap-3 px-8 py-4 rounded-xl bg-gradient-to-r from-indigo-600 to-purple-600 text-white font-semibold shadow-lg hover:shadow-xl transform hover:-translate-y-1 focus:outline-none focus:ring-4 focus:ring-purple-300 transition-all duration-300">
<span>Submit Survey</span>
<svg class="w-5 h-5 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"/></svg>
</button>
</div>
</form>
<div id="result" class="mt-8 text-center text-sm font-medium"></div>
</div>
<footer class="text-center text-white text-sm opacity-75 mt-8">
<p>Powered by GenSurvey</p>
</footer>
</main>
<script src="script.js"></script>
</body>
</html>
"#;

pub const QUESTION_FIELDSET: &str = r#"<fieldset id="{id}" class="question-card bg-white rounded-2xl border-2 border-gray-100 shadow-md hover:shadow-xl p-6 md:p-8 space-y-5 transition-all duration-300 hover:border-indigo-200" aria-labelledby="{id}-label">
<div id="{id}-label" class="text-xl font-bold flex items-start gap-2 mb-4 text-gray-800">
<span class="inline-block w-8 h-8 bg-gradient-to-br from-indigo-500 to-purple-500 rounded-lg flex items-center justify-center text-white text-sm font-bold shadow-sm">?</span>
<span class="flex-1">{text}{required_mark}</span>
</div>
<div class="options space-y-3" {options_attrs}>
{controls}
</div>
{subs}
</fieldset>
"#;

pub const SUB_QUESTION_BLOCK: &str = r#"<div class="subq border-l-4 border-indigo-400 bg-indigo-50 rounded-r-lg pl-6 pr-4 py-4 mt-4 space-y-3 transition-all duration-300{sub_hidden}" id="sub-{id}">
<div class="text-base font-semibold text-gray-800 flex items-center gap-2">
<svg class="w-4 h-4 text-indigo-500" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"/></svg>
{text}{required_mark}
</div>
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
