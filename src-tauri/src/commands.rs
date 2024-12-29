use crate::db::{connect_postgres, connect_sqlite};
use tauri::command;

#[command]
pub async fn test_postgres_connection() -> Result<String, String> {
    connect_postgres()
        .await
        .map(|_| "PostgreSQL connection successful".to_string())
}

#[command]
pub async fn test_sqlite_connection() -> Result<String, String> {
    connect_sqlite()
        .await
        .map(|_| "SQLite connection successful".to_string())
}
