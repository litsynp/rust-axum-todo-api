use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use serde_json::json;
use sqlx::{Pool, Postgres};

use rust_todo_api::{auth, todo, user};

pub fn build_routes(pool: Pool<Postgres>) -> Router {
    let api_routes = Router::new()
        .nest(
            "/auth",
            Router::new().route("/tokens", post(auth::handlers::get_tokens)),
        )
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
        .nest(
            "/users",
            Router::new().route(
                "/",
                get(user::handlers::find_user_by_email).post(user::handlers::register_user),
            ),
        )
        .layer(Extension(pool));

    Router::new()
        .route("/health", get(|| async { Json(json!({ "status": "ok" })) }))
        .nest("/api", api_routes)
}
