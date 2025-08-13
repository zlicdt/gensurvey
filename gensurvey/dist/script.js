// Dynamic behavior + submission + conditional visibility logic
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
    if (!v.ok) { result.textContent = v.message; result.style.color = '#d00'; return; }
    const payload = collect();
    result.style.color = '#333';
    result.textContent = 'Submitting...';
    try {
      if (endpoint && endpoint !== 'null') {
        const res = await fetch(endpoint, { method:'POST', headers:{'Content-Type':'application/json'}, body: JSON.stringify(payload)});
        if (!res.ok) throw new Error('Server returned '+res.status);
        result.textContent = 'Submitted successfully.';
      } else {
        console.log('Survey submission (no endpoint configured)', payload);
        result.textContent = 'Submitted (console only demo).';
      }
    } catch(err){
      console.error(err);
      result.textContent = 'Submission failed.';
      result.style.color = '#d00';
    }
  });
})();
