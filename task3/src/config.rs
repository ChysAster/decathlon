pub const BASE_URL: &str = "https://api.openweathermap.org/data/2.5";
pub const DB_FILE: &str = "weather.db";

pub fn get_api_key() -> String {
    std::env::var("OPENWEATHER_API_KEY")
        .expect("Missing OPENWEATHER_API_KEY in .env file")
}