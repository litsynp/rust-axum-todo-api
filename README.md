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
- chrono
- utoipa

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

## Swagger / OpenAPI

Access the Swagger UI at [`/swagger-ui`](http://localhost:8000/swagger-ui) and the OpenAPI spec
at [`/api-docs/openapi.json`](http://localhost:8000/api-docs/openapi.json).

You can also use RapiDoc at [`/rapidoc`](http://localhost:8000/rapidoc).

## Troubleshooting

- https://stackoverflow.com/questions/61561165/how-do-i-define-a-datetime-field-in-sqlx-rust

- https://stackoverflow.com/questions/75640266/axum-query-extractor-allow-for-struct-with-optional-fields-to-be-parsed

## CLI

This project includes a CLI to add users to the database.

```sh
$ cargo run --bin add-user -- --email=user@example.com --nickname=user --password=1234
Successfully registered user User { id: 1, email: "user@example.com", password: "1234", nickname: "user", created_at: 2024-04-19T17:14:54.358424Z, updated_at: 2024-04-19T17:14:54.358424Z, deleted_at: None }
```
