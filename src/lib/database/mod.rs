use sqlx::{postgres::{PgPoolOptions}, Pool, Postgres};
pub mod models;
pub struct Database {
    pub pool: Pool<Postgres>
}

impl Database {
    pub async fn new(uri: &str) -> Self {
        Self {
            pool: PgPoolOptions::new().connect(uri).await.unwrap()
        }
    }
}
