# rust-axum-todo-api

A Rust example of a todo application with axum and sqlx.

## Requirements

- Rust
- Postgres
- sqlx
- sqlx-cli
- Docker (optional)

## Dependencies

- axum
- tokio
- sqlx
- sqlx-cli
- serde
- serde_json
- dotenv

## Run the Application

### Docker

```sh
$ docker-compose up -d
```

### Database Setup

```sh
$ sqlx database create # create database
$ sqlx migrate run # run migrations
```

### Run (Development)

```sh
$ cargo run
```

### Run (Production)

```sh
$ cargo build --release
$ ./target/release/rust-todo-api
```

## Troubleshooting

- https://stackoverflow.com/questions/61561165/how-do-i-define-a-datetime-field-in-sqlx-rust

- https://stackoverflow.com/questions/75640266/axum-query-extractor-allow-for-struct-with-optional-fields-to-be-parsed
