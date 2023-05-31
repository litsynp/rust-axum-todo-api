use axum::{
    middleware::{self},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use sqlx::{Pool, Postgres};

use rust_todo_api::{
    auth::{self, models::JWT_SECRET},
    common::middlewares::{auth_middleware, AuthState},
    todo, user,
};

pub fn build_routes(pool: Pool<Postgres>) -> Router {
    let auth_state = AuthState {
        pool,
        jwt_secret: JWT_SECRET.to_string(),
    };

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
                )
                .route_layer(middleware::from_fn_with_state(
                    auth_state.clone(),
                    auth_middleware,
                )),
        )
        .nest(
            "/users",
            Router::new()
                .route(
                    "/",
                    get(user::handlers::find_user_by_email).post(user::handlers::register_user),
                )
                .route_layer(middleware::from_fn_with_state(
                    auth_state.clone(),
                    auth_middleware,
                )),
        );

    Router::new()
        .route("/health", get(|| async { Json(json!({ "status": "ok" })) }))
        .nest("/api", api_routes.with_state(auth_state))
}
