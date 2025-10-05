# decathlon

# Task 1

Get daily weather forecast for 10 favorite Belgian cities.

## Setup

Create a `.env` file with your OpenWeatherMap API key:
```
OPENWEATHER_API_KEY=your_api_key
```

## Commands

### Get Weather for All Cities
```bash
cargo run
```

This will print the current weather for all 10 predefined Belgian cities:
- Brussels
- Antwerp
- Ghent
- Bruges
- Leuven
- Liege
- Namur
- Mons
- Evere
- Avelgem

## Example Output
```
Brussels: 15.2°C, clear sky
Antwerp: 14.8°C, few clouds
Ghent: 15.0°C, scattered clouds
...
```

# Task 2

Command-line interface for weather queries with forecast support.

## Setup

Create a `.env` file with your OpenWeatherMap API key:
```
OPENWEATHER_API_KEY=your_api_key
```

## Commands

### Get Weather for All Favorite Cities
```bash
cargo run -- all
```

### Get Weather for a Specific Favorite City
```bash
cargo run -- city Brussels
cargo run -- city Antwerp
cargo run -- city Ghent
```

### Get Weather for Any City (Not in Favorites)
```bash
cargo run -- custom Paris
cargo run -- custom London
cargo run -- custom Tokyo
```

### Get Weather Forecast
```bash
# Get 3-day forecast (default)
cargo run -- forecast Brussels

# Get 5-day forecast
cargo run -- forecast Ghent --days 5

# Get forecast for tomorrow and day after tomorrow
cargo run -- forecast Antwerp --days 3
```

## Help
```bash
# Show all available commands
cargo run -- --help

# Show help for a specific command
cargo run -- forecast --help
```

## Favorite Cities List
- Brussels
- Antwerp
- Ghent
- Bruges
- Leuven
- Liege
- Namur
- Mons
- Evere
- Avelgem

# Task 3

Weather application with SQLite database for managing favorite cities and storing weather history.

## Setup

Create a `.env` file with your OpenWeatherMap API key:
```
OPENWEATHER_API_KEY=your_api_key
```

## Commands

### City Management

#### Add a City to Favorites
```bash
cargo run -- add-city Brussels
cargo run -- add-city Antwerp
cargo run -- add-city Ghent
```

#### Remove a City from Favorites
```bash
cargo run -- remove-city Brussels
cargo run -- remove-city Antwerp
```

#### List All Favorite Cities
```bash
cargo run -- list-cities
```

### Weather Queries

#### Get Weather for All Favorite Cities
```bash
cargo run -- all
```
*Automatically saves weather data to the database*

#### Get Weather for a Specific Favorite City
```bash
cargo run -- city Brussels
cargo run -- city Antwerp
```
*Automatically saves weather data to the database*

#### Get Weather for Any City (Without Saving)
```bash
cargo run -- custom Paris
cargo run -- custom London
```

### Weather History

#### View Weather History for a City
```bash
# Show last 10 records (default)
cargo run -- history Brussels

# Show last 20 records
cargo run -- history Antwerp --limit 20

# Show last 5 records
cargo run -- history Ghent --limit 5
```

### Forecast

#### Get Weather Forecast
```bash
# Get 3-day forecast (default)
cargo run -- forecast Brussels

# Get 5-day forecast
cargo run -- forecast Ghent --days 5
```
*Forecast data is not saved to the database*

## Help
```bash
# Show all available commands
cargo run -- --help

# Show help for a specific command
cargo run -- history --help
```

## Database

Weather data is stored in `weather.db` (SQLite) in the project directory.

The database contains:
- **cities**: Your favorite cities list
- **weather_records**: Historical weather data with timestamps