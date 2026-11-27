use clap::Parser;

mod utils;
mod weather;

#[derive(Parser, Debug)]
#[command(version, about = "A weather forecasting CLI")]
struct ForecastCLI {
    // The city name, zip code, or location you want to use for weather, ex: 47906
    #[arg(required = true)]
    name: Vec<String>, // spaces (like in west lafayette) are a bit tricky, so it requires a bit more work.
}

#[tokio::main]
async fn main() {
    let cli = ForecastCLI::parse();
    let search = cli.name.join(" ");
    let result = weather::cascade_fetch_weather(&search).await;

    if result.is_err() {
        utils::display(1, result.unwrap_err().to_string());
    } else {
        println!("{:#?}", result.unwrap())
    }
}
