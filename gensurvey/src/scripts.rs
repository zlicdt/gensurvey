//
//  scripts.rs
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
pub const SURVEY_SCRIPT: &str = r#"// Dynamic behavior + submission + conditional visibility logic
(function(){
  const form = document.getElementById('survey-form');
  if(!form) return;
  const result = document.getElementById('result');
  let endpoint = form.getAttribute('data-endpoint') || '';

  function collect(){
    const data = {};
    const formData = new FormData(form);
    for (const [k,v] of formData.entries()) {
      if (data[k]) {
        if (Array.isArray(data[k])) data[k].push(v); else data[k] = [data[k], v];
      } else data[k] = v;
    }
    return data;
  }

  function validate(){
    const requiredFields = form.querySelectorAll('[required]');
    for (const el of requiredFields) {
      if (el.type === 'radio') {
        const name = el.name;
        if (!form.querySelector(`input[type=radio][name='${name}']:checked`)) {
          return { ok:false, message: 'Please answer required single choice question: '+name };
        }
      } else if (el.type === 'checkbox') {
        const group = el.name;
        const groupContainer = el.closest('.options');
        if (groupContainer && groupContainer.hasAttribute('data-required-group')) {
          if (!form.querySelectorAll(`input[type=checkbox][name='${group}']:checked`).length) {
            return { ok:false, message: 'Please select at least one option for: '+group };
          }
        }
      } else if (!el.value.trim()) {
        return { ok:false, message: 'Please fill required field: '+el.name };
      }
    }
    return { ok:true };
  }

  function handleConditional(){
    const markers = form.querySelectorAll('input[data-display-parent]');
    markers.forEach(m => {
      const parent = m.getAttribute('data-display-parent');
      const val = m.getAttribute('data-display-value');
      const container = m.closest('.subq');
      if (!container) return;
      const parentInputs = form.querySelectorAll(`[name='${parent}']`);
      let show = false;
      parentInputs.forEach(pi => {
        if ((pi.type === 'radio' || pi.type === 'checkbox') && pi.checked && pi.value === val) show = true;
        if (pi.type === 'text' && pi.value === val) show = true;
      });
      container.classList.toggle('hidden', !show);
    });
  }

  form.addEventListener('change', handleConditional);
  form.addEventListener('input', handleConditional);
  handleConditional();

  form.addEventListener('submit', async (e) => {
    e.preventDefault();
    const v = validate();
    if (!v.ok) { 
      result.innerHTML = '<div class="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-red-50 border-2 border-red-200 text-red-700"><svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/></svg><span>' + v.message + '</span></div>';
      result.scrollIntoView({ behavior: 'smooth', block: 'center' });
      return; 
    }
    const payload = collect();
    result.innerHTML = '<div class="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-indigo-50 border-2 border-indigo-200 text-indigo-700"><svg class="animate-spin w-5 h-5" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg><span>Submitting your response...</span></div>';
    try {
      if (endpoint && endpoint !== 'null') {
        const res = await fetch(endpoint, { method:'POST', headers:{'Content-Type':'application/json'}, body: JSON.stringify(payload)});
        if (!res.ok) throw new Error('Server returned '+res.status);
        result.innerHTML = '<div class="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-green-50 border-2 border-green-200 text-green-700 animate-pulse"><svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/></svg><span>Successfully submitted! Thank you for your response.</span></div>';
        form.style.opacity = '0.6';
        form.style.pointerEvents = 'none';
      } else {
        console.log('Survey submission (no endpoint configured)', payload);
        result.innerHTML = '<div class="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-blue-50 border-2 border-blue-200 text-blue-700"><svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/></svg><span>Demo mode: Response logged to console.</span></div>';
      }
      result.scrollIntoView({ behavior: 'smooth', block: 'center' });
    } catch(err){
      console.error(err);
      result.innerHTML = '<div class="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-red-50 border-2 border-red-200 text-red-700"><svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/></svg><span>Submission failed. Please try again.</span></div>';
      result.scrollIntoView({ behavior: 'smooth', block: 'center' });
    }
  });
})();
"#;
