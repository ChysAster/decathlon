use sqlx::{sqlite::SqlitePool};
use crate::config::DB_FILE;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePool::connect(&format!("sqlite://{}?mode=rwc", DB_FILE)).await?;

    // Create tables
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS cities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL
        )
        "#
    )
        .execute(&pool)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS weather_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            city_id INTEGER NOT NULL,
            temperature REAL NOT NULL,
            description TEXT NOT NULL,
            recorded_at TEXT NOT NULL,
            FOREIGN KEY (city_id) REFERENCES cities(id) ON DELETE CASCADE
        )
        "#
    )
        .execute(&pool)
        .await?;

    Ok(pool)
}