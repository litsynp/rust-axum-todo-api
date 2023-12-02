use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

use tracing::info;

use crate::routes;
use rust_todo_api::infrastructure::database;

pub async fn create_server() {
    tracing_subscriber::fmt::init();

    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = database::get_postgres_pool(db_url.as_str())
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Failed to connect to Postgres with provided URL: {}",
                db_url
            )
        });

    database::migrate(&pool).await;

    let router = routes::build_routes(pool);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on {}", addr);

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
