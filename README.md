# rust-axum-todo-api

A Rust example of a todo application with axum and sqlx.

## Requirements

- Rust 1.83+
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
- askama (for frontend with tailwindcss)

## Run the Application

### Docker

```sh
$ make db:up  # make db:down to stop and db:destroy to remove
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

If you want to watch for file changes, you can use `cargo watch`:

```sh
$ make watch:install  # Install cargo-watch
$ make watch
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

## Frontend

<img src="https://private-user-images.githubusercontent.com/42485462/324154929-951b9588-7999-4234-b2ab-9644e9f31600.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MTM1ODk0MDUsIm5iZiI6MTcxMzU4OTEwNSwicGF0aCI6Ii80MjQ4NTQ2Mi8zMjQxNTQ5MjktOTUxYjk1ODgtNzk5OS00MjM0LWIyYWItOTY0NGU5ZjMxNjAwLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNDA0MjAlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjQwNDIwVDA0NTgyNVomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTgzMmIxMjYxZThlZGE4OGIyYTU1NjE3Y2Q2ZmY1MTUzYWIxNmIwNGVmNGJkZmFhN2YwMzdmMjk3Y2M2MjA3NzgmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JmFjdG9yX2lkPTAma2V5X2lkPTAmcmVwb19pZD0wIn0.F40j2WDIzQiFheaTgm9Dt8fk7IBcIGjqT7aq-dDPPJg" width="400" />

This project uses askama to render HTML templates.

If you need to access the frontend, you can run the following command:

```sh
$ npm install
$ npm run tailwind:build # or npm run tailwind:watch to watch for changes
```

Go to index page at [`/`](http://localhost:8000/) and login
at [`/login`](http://localhost:8000/login).

You can add, edit and delete todos.
