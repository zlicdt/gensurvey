# gensurvey

<p align="left">
<img alt="License" src="https://img.shields.io/github/license/zlicdt/gensurvey?label=License" height=22.5>
</p>

A small static survey page generator from a JSONC specification.

## Features

- JSON / JSONC survey specification parsing (supports `//` line comments)
- Question types: single choice, multiple choice, text
- Required field validation (client-side)
- Nested (sub) questions with conditional display based on a parent question's value
- Generates a static webpage (no external assets)
- Optional server endpoint POST submission if `gensurvey_server` is provided in the spec

## Spec Example
See `scaffold/example.jsonc` for structure:

```jsonc
{
  "title": "Survey Title",
  "description": "Survey Description",
  "gensurvey_server": "http://localhost:11451", // Use with gensurvey server
  "questions": [ /* ... */ ]
}
```

## Test

Follow the component's README.

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

## License
GNU AFFERO GENERAL PUBLIC LICENSE 3.0
