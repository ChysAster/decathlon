pub const BASE_URL: &str = "https://api.openweathermap.org/data/2.5";

pub const FAVORITE_CITIES: [&str; 10] = [
    "Brussels", "Antwerp", "Ghent", "Bruges", "Leuven",
    "Liege", "Namur", "Mons", "Evere", "Avelgem"
];

pub fn get_api_key() -> String {
    std::env::var("OPENWEATHER_API_KEY")
        .expect("Missing OPENWEATHER_API_KEY in .env file")
}