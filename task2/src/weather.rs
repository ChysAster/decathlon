use crate::config::{get_api_key, BASE_URL};
use crate::models::{ForecastResponse, WeatherInfo, WeatherResponse};

pub async fn get_weather(city: &str) -> Result<WeatherInfo, Box<dyn std::error::Error>> {
    let api_key = get_api_key();

    let url = format!(
        "{}/weather?q={}&appid={}&units=metric",
        BASE_URL, city, api_key
    );

    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        return Err(format!("API error ({}): {}", status, error_text).into());
    }

    let resp: WeatherResponse = response.json().await?;
    Ok(WeatherInfo {
        temp: resp.main.temp,
        description: resp.weather[0].description.clone(),
    })
}

pub async fn get_forecast(city: &str, days: u8) -> Result<Vec<WeatherInfo>, Box<dyn std::error::Error>> {
    let api_key = get_api_key();

    let url = format!(
        "{}/forecast?q={}&appid={}&units=metric",
        BASE_URL, city, api_key
    );

    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        return Err(format!("API error ({}): {}", status, error_text).into());
    }

    let resp: ForecastResponse = response.json().await?;

    // Group by date and take first forecast of each day
    let mut daily_forecasts = Vec::new();
    let mut last_date = String::new();

    for item in resp.list {
        let date = item.dt_txt.split(' ').next().unwrap_or("");

        if date != last_date && daily_forecasts.len() < days as usize {
            daily_forecasts.push(WeatherInfo {
                temp: item.main.temp,
                description: item.weather[0].description.clone(),
            });
            last_date = date.to_string();
        }
    }

    Ok(daily_forecasts)
}