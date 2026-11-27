use clap::Parser;
use comfy_table::Table;

mod clothing;
mod utils;
mod weather;

#[derive(Parser, Debug)]
#[command(version, about = "A simple weather forecasting and clothes recommending CLI")]
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
            let clothing_recs = clothing::decide_recommendation(result);
            let accessories = if clothing_recs.accessories.is_empty() {
                "None recommended".to_string()
            } else {
                clothing_recs.accessories.join(", ")
            };
            let mut recs_table = Table::new();
            recs_table
                .set_header(vec!["Item", "Recommendation"])
                .add_row(vec!["Top", &clothing_recs.top])
                .add_row(vec!["Outerwear", &clothing_recs.outerwear])
                .add_row(vec!["Bottom", &clothing_recs.bottom])
                .add_row(vec!["Footwear", &clothing_recs.footwear])
                .add_row(vec!["Accessories", &accessories]);
            println!("{recs_table}");
            println!("Notes: {}", clothing_recs.notes.join(", "));
        }
        Err(err) => utils::display(1, err.to_string()),
    }
}
