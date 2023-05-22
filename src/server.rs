use std::net::SocketAddr;

use axum::{routing::get, Extension, Json, Router};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::infrastructure::database;
use crate::todo;

pub async fn create_server() {
    // Get db_url from .env
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = database::get_postgres_pool(db_url.as_str()).await.expect(
        format!(
            "Failed to connect to Postgres with provided URL: {}",
            db_url
        )
        .as_str(),
    );

    let router = build_routes(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

fn build_routes(pool: Pool<Postgres>) -> Router {
    let api_routes = Router::new()
        .nest(
            "/todos",
            Router::new()
                .route(
                    "/",
                    get(todo::handlers::find_todos).post(todo::handlers::create_todo),
                )
                .route(
                    "/:id",
                    get(todo::handlers::find_todo_by_id)
                        .put(todo::handlers::edit_todo_by_id)
                        .delete(todo::handlers::delete_todo_by_id),
                ),
        )
        .layer(Extension(pool));

    Router::new()
        .route("/health", get(|| async { Json(json!({ "status": "ok" })) }))
        .nest("/api", api_routes)
}
