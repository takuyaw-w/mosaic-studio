use sqlx::{Pool, Postgres, Sqlite};
use dotenvy::dotenv;
use std::env;

pub async fn connect_postgres() -> Result<Pool<Postgres>, String> {
    dotenv().ok();

    let url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set".to_string())?;

    Pool::<Postgres>::connect(&url)
        .await
        .map_err(|e| format!("Failed to connect to Postgres: {}", e))
}

pub async fn connect_sqlite() -> Result<Pool<Sqlite>, String> {
    dotenv().ok();

let url = env::var("SQLITE_URL").map_err(|_| "SQLITE_URL must be set".to_string())?;

    Pool::<Sqlite>::connect(&url)
        .await
        .map_err(|e| format!("Failed to connect to SQLite: {}", e))
}
