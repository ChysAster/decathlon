use sqlx::SqlitePool;
use chrono::Utc;
use crate::models::{City, WeatherInfo, WeatherRecord};

pub struct Repository {
    pool: SqlitePool,
}

impl Repository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn add_city(&self, name: &str) -> Result<i64, sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        let result = sqlx::query(
            "INSERT INTO cities (name, created_at) VALUES (?, ?)"
        )
            .bind(name)
            .bind(now)
            .execute(&self.pool)
            .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn remove_city(&self, name: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM cities WHERE name = ?")
            .bind(name)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_all_cities(&self) -> Result<Vec<City>, sqlx::Error> {
        let cities = sqlx::query_as::<_, (i64, String, String)>(
            "SELECT id, name, created_at FROM cities ORDER BY name"
        )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|(id, name, created_at)| City { id, name, created_at })
            .collect();

        Ok(cities)
    }

    pub async fn get_city_by_name(&self, name: &str) -> Result<Option<City>, sqlx::Error> {
        let result = sqlx::query_as::<_, (i64, String, String)>(
            "SELECT id, name, created_at FROM cities WHERE name = ? COLLATE NOCASE"
        )
            .bind(name)
            .fetch_optional(&self.pool)
            .await?
            .map(|(id, name, created_at)| City { id, name, created_at });

        Ok(result)
    }

    pub async fn save_weather_record(
        &self,
        city_id: i64,
        weather: &WeatherInfo,
    ) -> Result<i64, sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        let result = sqlx::query(
            "INSERT INTO weather_records (city_id, temperature, description, recorded_at)
             VALUES (?, ?, ?, ?)"
        )
            .bind(city_id)
            .bind(weather.temp)
            .bind(&weather.description)
            .bind(now)
            .execute(&self.pool)
            .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_weather_history(
        &self,
        city_id: i64,
        limit: i64,
    ) -> Result<Vec<WeatherRecord>, sqlx::Error> {
        let records = sqlx::query_as::<_, (i64, i64, f64, String, String)>(
            "SELECT id, city_id, temperature, description, recorded_at
             FROM weather_records
             WHERE city_id = ?
             ORDER BY recorded_at DESC
             LIMIT ?"
        )
            .bind(city_id)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|(id, city_id, temperature, description, recorded_at)| WeatherRecord {
                id,
                city_id,
                temperature,
                description,
                recorded_at,
            })
            .collect();

        Ok(records)
    }
}
