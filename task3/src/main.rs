mod cli;
mod config;
mod db;
mod models;
mod repository;
mod weather;

use clap::Parser;
use cli::{Cli, Commands};
use db::init_db;
use repository::Repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let pool = init_db().await?;
    let repo = Repository::new(pool);

    let cli = Cli::parse();

    match cli.command {
        Commands::All => {
            println!("Weather for all favorite cities:\n");
            let cities = repo.get_all_cities().await?;

            if cities.is_empty() {
                println!("No favorite cities found. Add some with 'add-city' command.");
                return Ok(());
            }

            for city in cities {
                match weather::get_weather(&city.name).await {
                    Ok(info) => {
                        println!("{}: {}°C, {}", city.name, info.temp, info.description);
                        // Store in database
                        if let Err(e) = repo.save_weather_record(city.id, &info).await {
                            eprintln!("Failed to save weather for {}: {}", city.name, e);
                        }
                    }
                    Err(e) => eprintln!("Error fetching {}: {}", city.name, e),
                }
            }
        }
        Commands::City { name } => {
            match repo.get_city_by_name(&name).await? {
                Some(city) => {
                    match weather::get_weather(&city.name).await {
                        Ok(info) => {
                            println!("{}: {}°C, {}", city.name, info.temp, info.description);
                            repo.save_weather_record(city.id, &info).await?;
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                None => {
                    eprintln!("'{}' is not in your favorite cities.", name);
                    println!("Add it with: cargo run -- add-city {}", name);
                }
            }
        }
        Commands::Custom { name } => {
            match weather::get_weather(&name).await {
                Ok(info) => println!("{}: {}°C, {}", name, info.temp, info.description),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Forecast { city, days } => {
            match weather::get_forecast(&city, days).await {
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
        Commands::AddCity { name } => {
            match repo.add_city(&name).await {
                Ok(_) => println!("✓ Added '{}' to favorite cities", name),
                Err(e) => eprintln!("Error adding city: {}", e),
            }
        }
        Commands::RemoveCity { name } => {
            match repo.remove_city(&name).await {
                Ok(deleted) => {
                    if deleted {
                        println!("✓ Removed '{}' from favorite cities", name);
                    } else {
                        println!("'{}' was not in your favorites", name);
                    }
                }
                Err(e) => eprintln!("Error removing city: {}", e),
            }
        }
        Commands::ListCities => {
            let cities = repo.get_all_cities().await?;
            if cities.is_empty() {
                println!("No favorite cities yet.");
            } else {
                println!("Favorite cities:");
                for city in cities {
                    println!("  - {}", city.name);
                }
            }
        }
        Commands::History { name, limit } => {
            match repo.get_city_by_name(&name).await? {
                Some(city) => {
                    let records = repo.get_weather_history(city.id, limit).await?;
                    if records.is_empty() {
                        println!("No weather history for {}", name);
                    } else {
                        println!("Weather history for {}:\n", name);
                        for record in records {
                            println!("{}: {}°C, {}",
                                     record.recorded_at,
                                     record.temperature,
                                     record.description
                            );
                        }
                    }
                }
                None => eprintln!("'{}' is not in your favorite cities", name),
            }
        }
    }

    Ok(())
}