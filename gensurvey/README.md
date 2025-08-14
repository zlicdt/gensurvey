# gensurvey

Simple static survey page generator.

## Usage

Build:

```
cargo build --release
```

Generate survey HTML into a NEW output directory:

```
./target/release/gensurvey -i path/to/survey.jsonc -o dist
```

This will create `dist/` (must not already exist) and write `index.html` + `script.js`.

Options:

- `-i, --input <PATH>`: Survey specification file (supports `//` comments).
- `-o, --output <DIR>`: Output directory to create (must not exist).
- `-h, --help`: Show help.

## Survey Spec Fields

Please refer to https://github.com/zlicdt/gensurvey/blob/main/scaffold/example.jsonc for the format.

- `title`: String
- `description`: String
- `gensurvey_server`: Optional base URL of server; `/submit` appended for POST endpoint.
- `questions`: Array of questions.

Question object:

- `id`: Unique string
- `text`: Prompt text
- `type`: `single_choice` | `multiple_choice` | `text`
- `options`: For choice questions, array of `{ value, label }`
- `required`: Boolean
- `sub_questions`: Optional array of sub question objects (same shape) plus:
  - `hide`: Start hidden
  - `when_display`: `{ condition, value }` conditional display rule

## License

GNU AFFERO GENERAL PUBLIC LICENSE 3.0
