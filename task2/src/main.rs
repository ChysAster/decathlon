mod cli;
mod config;
mod models;
mod weather;

use clap::Parser;
use cli::{Cli, Commands};
use config::FAVORITE_CITIES;
use weather::{get_forecast, get_weather};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::All => {
            println!("Weather for all favorite cities:\n");
            for city in FAVORITE_CITIES {
                match get_weather(city).await {
                    Ok(info) => println!("{}: {}°C, {}", city, info.temp, info.description),
                    Err(e) => eprintln!("Error fetching {}: {}", city, e),
                }
            }
        }
        Commands::City { name } => {
            if FAVORITE_CITIES.iter().any(|&c| c.eq_ignore_ascii_case(&name)) {
                match get_weather(&name).await {
                    Ok(info) => println!("{}: {}°C, {}", name, info.temp, info.description),
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else {
                eprintln!("'{}' is not in your favorite cities list.", name);
                println!("Available cities: {}", FAVORITE_CITIES.join(", "));
            }
        }
        Commands::Custom { name } => {
            match get_weather(&name).await {
                Ok(info) => println!("{}: {}°C, {}", name, info.temp, info.description),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Forecast { city, days } => {
            match get_forecast(&city, days).await {
                Ok(forecasts) => {
                    println!("Forecast for {}:\n", city);
                    for (day, info) in forecasts.iter().enumerate() {
                        let day_label = match day {
                            0 => "Today",
                            1 => "Tomorrow",
                            2 => "Day after tomorrow",
                            _ => "Future",
                        };
                        println!("{}: {}°C, {}", day_label, info.temp, info.description);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }

    Ok(())
}
