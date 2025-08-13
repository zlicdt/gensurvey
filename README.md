# gensurvey

A small static survey page generator from a JSONC specification.

## Features

- JSON / JSONC survey specification parsing (supports `//` line comments)
- Question types: single choice, multiple choice, text
- Required field validation (client-side)
- Nested (sub) questions with conditional display based on a parent question's value
- Generates a standalone `dist/index.html` (no external assets) suitable for serving via nginx or any static host
- Optional server endpoint POST submission if `gensurvey_server` is provided in the spec

## Spec Example
See `scaffold/example.jsonc` for structure:

```jsonc
{
  "title": "Survey Title",
  "description": "Survey Description",
  "gensurvey_server": "http://localhost:11451",
  "questions": [ /* ... */ ]
}
```

## Build & Generate

From the crate directory:

```bash
cargo run -- ../example.jsonc
```

Outputs: `dist/index.html`.

## Serve with nginx

Example nginx server block:

```
server {
    listen 80;
    server_name survey.local;
    root /path/to/gensurvey/gensurvey/dist;
    index index.html;
    location / { try_files $uri $uri/ =404; }
}
```

Reload nginx and open http://survey.local/.

## Extending

- Add new question types: extend `QuestionType` enum and adjust `render_question` logic.
- Add persistent storage: implement a real HTTP backend to receive submissions at `gensurvey_server` URL.
- Add accessibility improvements: ARIA roles, focus management.

## License
GNU AFFERO GENERAL PUBLIC LICENSE 3.0
