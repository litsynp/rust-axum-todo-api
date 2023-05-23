use std::net::SocketAddr;

use crate::infrastructure::database;
use crate::routes;

pub async fn create_server() {
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
