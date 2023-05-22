use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn get_postgres_pool(db_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
}
