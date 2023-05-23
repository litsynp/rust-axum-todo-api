use std::net::SocketAddr;

use crate::infrastructure::database;
use crate::routes;

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

    let router = routes::build_routes(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
