use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::env;

pub type DbPool = SqlitePool;

pub async fn init_db() -> anyhow::Result<DbPool> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./portfolio_tracker.db".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}
