
use clap::Parser;
use colored::Colorize;
use comfy_table::{Cell, Color, Table};

mod clothing;
mod utils;
mod weather;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "A simple weather forecasting and clothes recommending CLI"
)]
struct ForecastCLI {
    // The city name, zip code, or location you want to use for weather, ex: 47906
    #[arg(required = true)]
    name: Vec<String>, // spaces (like in west lafayette) are a bit tricky, so it requires a bit more work.
}

#[tokio::main]
async fn main() {
    let cli = ForecastCLI::parse();
    let search = cli.name.join(" ");

    match weather::cascade_fetch_weather(&search).await {
        Ok(result) => {
            // give the clothing rules their own copy so we can still use the forecast in the table
            let clothing_recs = clothing::decide_recommendation(result.to_owned());

            let accessories = if clothing_recs.accessories.is_empty() {
                "None recommended".to_string()
            } else {
                clothing_recs.accessories.join(", ")
            };

            let mut recs_table = Table::new();

            recs_table
                .load_style(comfy_table::presets::UTF8_FULL)
                .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
                .set_header(vec![
                    Cell::new("CLOTHING RECOMMENDATIONS")
                        .fg(Color::Yellow)
                        .add_attribute(comfy_table::Attribute::Bold),
                ])
                .add_row(vec![
                    Cell::new("Top").fg(Color::Cyan),
                    Cell::new(&clothing_recs.top).fg(Color::Green),
                ])
                .add_row(vec![
                    Cell::new("Outerwear").fg(Color::Cyan),
                    Cell::new(&clothing_recs.outerwear).fg(Color::Green),
                ])
                .add_row(vec![
                    Cell::new("Bottom").fg(Color::Cyan),
                    Cell::new(&clothing_recs.bottom).fg(Color::Green),
                ])
                .add_row(vec![
                    Cell::new("Footwear").fg(Color::Cyan),
                    Cell::new(&clothing_recs.footwear).fg(Color::Green),
                ])
                .add_row(vec![
                    Cell::new("Accessories").fg(Color::Cyan),
                    Cell::new(&accessories).fg(Color::Green),
                ]);

            let conditions = match result.weather_code {
                0 => "☀️ Clear sky",
                1 => "🌤️ Mainly clear",
                2 => "⛅ Partly cloudy",
                3 => "☁️ Overcast",
                45 | 48 => "🌫️ Foggy",
                51 | 53 | 55 => "🌦️ Drizzle",
                56 | 57 => "🧊 Freezing drizzle",
                61 | 63 | 65 => "🌧️ Rain",
                66 | 67 => "🧊 Freezing rain",
                71 | 73 | 75 | 77 => "❄️ Snow",
                80 | 81 | 82 => "🌦️ Rain showers",
                85 | 86 => "🌨️ Snow showers",
                95 | 96 | 99 => "⛈️ Thunderstorms",
                // an unfamiliar code can still come with a usable forecast
                _ => "❓ Unknown conditions",
            };
            
            println!("\n{}", "🌤️ TOMORROW'S FORECAST".cyan().bold());
            let mut forecast_table = Table::new();

            forecast_table
                .load_style(comfy_table::presets::NOTHING)
                .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
                .add_row(vec!["Date:", &result.date])
                .add_row(vec![
                    "Temperature:",
                    &format!("{:.0}°F / {:.0}°F", result.high, result.low),
                ])
                .add_row(vec![
                    "Feels like:",
                    &format!(
                        "{:.0}°F / {:.0}°F",
                        result.feels_like_high, result.feels_like_low
                    ),
                ])
                .add_row(vec![
                    "Wind:",
                    &format!("{:.1} mph", result.wind_speed_max),
                ])
                .add_row(vec![
                    "Precipitation:",
                    &format!("{}%", result.precipitation_probability),
                ])
                .add_row(vec!["Conditions:", conditions]);

            println!("{forecast_table}");
            println!("\n{recs_table}");

            if !clothing_recs.notes.is_empty() {
                println!("\n{}", "NOTES".yellow().bold());

                for note in &clothing_recs.notes {
                    println!("{} {}", "•".yellow(), note);
                }
            }
        }
        Err(err) => utils::display(1, err.to_string()),
    }
}
