use reqwest;
use serde::Deserialize;

const BASE_URL: &str = "https://api.openweathermap.org/data/2.5";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let cities = vec![
        "Brussels", "Antwerp", "Ghent", "Bruges", "Leuven",
        "Liege", "Namur", "Mons", "Evere", "Avelgem"
    ];

    for city in cities {
        match get_weather(city).await {
            Ok(info) => println!("{}: {}°C, {}", city, info.temp, info.description),
            Err(e) => eprintln!("Error fetching {}: {}", city, e),
        }
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
}

#[derive(Debug, Deserialize)]
struct Weather {
    description: String,
}

struct WeatherInfo {
    temp: f64,
    description: String,
}

async fn get_weather(city: &str) -> Result<WeatherInfo, reqwest::Error> {
    let api_key = std::env::var("OPENWEATHER_API_KEY")
        .expect("Missing api key in .env file");

    let path = format!("/weather?q={}&appid={}&units=metric", city, api_key);
    let url = format!("{}{}", BASE_URL, path);

    let resp: WeatherResponse = reqwest::get(&url).await?.json().await?;
    Ok(WeatherInfo {
        temp: resp.main.temp,
        description: resp.weather[0].description.clone(),
    })
}
