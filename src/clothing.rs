use crate::weather;

#[derive(Debug)]
pub struct ClothingRecommendation {
    pub top: String,
    pub outerwear: String,
    pub bottom: String,
    pub footwear: String,
    pub accessories: Vec<String>,
    pub notes: Vec<String>,
}

pub fn decide_recommendation(forecast: weather::TomorrowForecast) -> ClothingRecommendation {
    // the feels-like midpoint gives us a starting point for the whole day, with cold lows handled below
    let feels_like_midpoint = (forecast.feels_like_low + forecast.feels_like_high) / 2.0;

    let top_recommendation = if feels_like_midpoint < 32.0 {
        "Thermal / long-sleeve shirt"
    } else if feels_like_midpoint < 45.0 {
        "Sweatshirt"
    } else if feels_like_midpoint < 60.0 {
        "Hoodie"
    } else if feels_like_midpoint < 75.0 {
        "T-shirt"
    } else {
        "Lightweight t-shirt"
    };

    // a mild midpoint can hide a cold low, so check both before settling for a windbreaker
    let outerwear_recommendation = if feels_like_midpoint < 20.0 {
        Some("Heavy winter jacket")
    } else if feels_like_midpoint < 35.0 {
        Some("Insulated jacket")
    } else if feels_like_midpoint < 45.0 {
        Some("Medium-weight jacket")
    } else if feels_like_midpoint < 55.0 {
        Some("Light jacket")
    } else if forecast.feels_like_low < 40.0 && feels_like_midpoint < 65.0 {
        Some("Light jacket")
    } else if forecast.wind_speed_max >= 20.0 && feels_like_midpoint < 70.0 {
        Some("Windbreaker")
    } else {
        None
    };

    let bottom_recommendation = if feels_like_midpoint < 25.0 {
        "Thermal / lined pants"
    } else if feels_like_midpoint < 60.0 {
        "Long pants / sweatpants"
    } else if forecast.feels_like_low < 45.0 {
        "Long pants"
    } else if feels_like_midpoint < 75.0 {
        "Lightweight pants / light joggers"
    } else {
        "Shorts"
    };

    let w_code = forecast.weather_code;
    // give snow and ice their own boots first, since a high chance of precip could include either
    let shoes_recommendation = if matches!(w_code, 71..=77 | 85..=86) {
        "Insulated water-resistant boots"
    } else if matches!(w_code, 56..=57 | 66..=67) {
        "Boots with good traction"
    } else if matches!(w_code, 51..=55 | 61..=65 | 80..=82)
        || forecast.precipitation_probability >= 60
    {
        "Water-resistant shoes"
    } else if forecast.feels_like_low < 15.0 {
        "Insulated shoes / boots"
    } else {
        "Regular sneakers / any shoes"
    };

    // use the low for cold-weather extras since someone might be out during the coldest part of the day
    let mut accessories_recommendation = Vec::new();
    if forecast.feels_like_low < 32.0 {
        accessories_recommendation.push("Gloves".to_string());
    }
    if forecast.feels_like_low < 20.0 {
        accessories_recommendation.push("Beanie / warm hat".to_string());
    }
    if forecast.feels_like_low < 10.0 {
        accessories_recommendation.push("Scarf".to_string());
    }
    // the precip chance can include snow too, so check for rain or drizzle before suggesting an umbrella
    if forecast.precipitation_probability >= 50 && matches!(w_code, 51..=55 | 61..=65 | 80..=82) {
        accessories_recommendation.push("Umbrella".to_string());
    }
    if forecast.wind_speed_max >= 20.0 && feels_like_midpoint < 50.0 {
        accessories_recommendation.push("Face covering".to_string());
    }

    let mut notes = Vec::new();
    if (forecast.low - forecast.feels_like_low) >= 10.0 {
        notes.push("Feels significantly colder than actual temperature".to_string());
    }
    if (forecast.feels_like_high - forecast.feels_like_low) >= 20.0 {
        notes.push("Large temperature swings possible, consider layering".to_string());
    }
    if forecast.wind_speed_max >= 20.0 {
        notes.push("Strong winds possible".to_string());
    }
    if forecast.precipitation_probability >= 60 {
        notes.push("Precipitation likely".to_string());
    }
    if forecast.feels_like_low < 32.0 {
        notes.push("Freezing possible".to_string());
    }

    ClothingRecommendation {
        top: top_recommendation.to_string(),
        outerwear: outerwear_recommendation
            .unwrap_or("No outerwear needed")
            .to_string(),
        bottom: bottom_recommendation.to_string(),
        footwear: shoes_recommendation.to_string(),
        accessories: accessories_recommendation,
        notes,
    }
}
