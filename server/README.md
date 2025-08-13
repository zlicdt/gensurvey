# gensurvey server

Server to receive or view submissions

## Usage

Build:

```
cargo build --release
```

Start in normal mode:

```
./target/release/gensurvey-server # use default port: 11451
```

Start in normal mode with custom port:

```
./target/release/gensurvey-server -p 19198 # use port: 19198
# or
./target/release/gensurvey-server --port 19198 # use port: 19198
```

Start in admin mode(have access to submissions view):

```
./target/release/gensurvey-server -A
# or
./target/release/gensurvey-server --admin_mode
```

In admin mode, you can access submissions view by link `http://host:port/submissions`, for example: `http://127.0.0.1:11451/submissions`, it returns a json file.

## License

GNU AFFERO GENERAL PUBLIC LICENSE 3.0
