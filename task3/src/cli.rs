use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "weather")]
#[command(about = "Get weather information", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    All,
    City {
        #[arg(help = "Name of the city from favorites")]
        name: String
    },
    Custom {
        #[arg(help = "Name of any city")]
        name: String
    },
    Forecast {
        #[arg(help = "Name of the city")]
        city: String,
        #[arg(short, long, default_value_t = 3, help = "Number of days to forecast")]
        days: u8,
    },
    AddCity {
        #[arg(help = "Name of the city to add")]
        name: String,
    },
    RemoveCity {
        #[arg(help = "Name of the city to remove")]
        name: String,
    },
    ListCities,
    History {
        #[arg(help = "Name of the city")]
        name: String,
        #[arg(short, long, default_value_t = 10, help = "Number of records to show")]
        limit: i64,
    },
}