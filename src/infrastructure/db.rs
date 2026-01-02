use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn new_pool(db_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
        .expect("failed to connect db")
}
