use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub type PgPoolSquad = PgPool;

pub async fn establish_connection(database_url: &str) -> Result<PgPoolSquad> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}
