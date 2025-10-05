use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub main: Main,
    pub weather: Vec<Weather>,
}

#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    pub list: Vec<ForecastItem>,
}

#[derive(Debug, Deserialize)]
pub struct ForecastItem {
    pub main: Main,
    pub weather: Vec<Weather>,
    pub dt_txt: String,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f64,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub description: String,
}

#[derive(Debug)]
pub struct WeatherInfo {
    pub temp: f64,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct City {
    pub id: i64,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug)]
pub struct WeatherRecord {
    pub id: i64,
    pub city_id: i64,
    pub temperature: f64,
    pub description: String,
    pub recorded_at: String,
}