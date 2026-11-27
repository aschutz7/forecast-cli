use reqwest::{Error, Result, get};
use serde::{Deserialize, Serialize};

struct ForecastDefaults {
    timezone: &'static str,
    forecast_days: u8,
    wind_speed_unit: &'static str,
    temperature_unit: &'static str,
    precipitation_unit: &'static str,
}

struct GeocodingDefaults {
    count: u8,              // 10
    language: &'static str, // en
    format: &'static str,   // json
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DailyOpts {
    #[serde(rename = "temperature_2m_max")]
    Temperature2mMax,
    #[serde(rename = "temperature_2m_min")]
    Temperature2mMin,
    #[serde(rename = "wind_speed_10m_max")]
    WindSpeed10mMax,

    WeatherCode,
    ApparentTemperatureMin,
    ApparentTemperatureMax,
    PrecipitationProbabilityMax,
}

impl DailyOpts {
    pub fn as_str(&self) -> String {
        serde_json::to_value(self)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HourlyOpts {
    #[serde(rename = "temperature_2m")]
    Temperature2m,
    #[serde(rename = "wind_speed_10m")]
    WindSpeed10m,

    ApparentTemperature,
    PrecipitationProbability,
    Precipitation,
    WeatherCode,
}

impl HourlyOpts {
    pub fn as_str(&self) -> String {
        serde_json::to_value(self)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeocodingRequestData {
    pub name: String,     // zip code or search content
    pub count: u8,        // 10
    pub language: String, // en
    pub format: String,   // json
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeocodingSearchResult {
    pub id: u64,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: Option<f64>,
    pub feature_code: Option<String>,
    pub country_code: Option<String>,
    pub admin1_id: Option<u64>,
    pub admin2_id: Option<u64>,
    pub timezone: Option<String>,
    pub population: Option<u32>,
    #[serde(default)] // empty array by default
    pub postcodes: Vec<String>,
    pub country_id: Option<u64>,
    pub country: Option<String>,
    pub admin1: Option<String>,
    pub admin2: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeocodingResponse {
    #[serde(default)] // once again, empty array by default to prevent unhandled cases
    pub results: Vec<GeocodingSearchResult>,
    pub generationtime_ms: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastRequestData {
    pub latitude: f64,
    pub longitude: f64,
    pub daily: Vec<DailyOpts>,
    pub hourly: Vec<HourlyOpts>,
    pub timezone: String,
    pub forecast_days: u8,
    pub wind_speed_unit: String,
    pub temperature_unit: String,
    pub precipitation_unit: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub hourly_units: HourlyUnits,
    pub hourly: HourlyForecast,
    pub daily_units: DailyUnits,
    pub daily: DailyForecast,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HourlyForecast {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub apparent_temperature: Vec<f64>,
    pub precipitation_probability: Vec<u8>,
    pub precipitation: Vec<f64>,
    pub weather_code: Vec<u8>,
    pub wind_speed_10m: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyForecast {
    pub time: Vec<String>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
    pub weather_code: Vec<u8>,
    pub apparent_temperature_min: Vec<f64>,
    pub apparent_temperature_max: Vec<f64>,
    pub wind_speed_10m_max: Vec<f64>,
    pub precipitation_probability_max: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HourlyUnits {
    pub time: String,
    pub temperature_2m: String,
    pub apparent_temperature: String,
    pub precipitation_probability: String,
    pub precipitation: String,
    pub weather_code: String,
    pub wind_speed_10m: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyUnits {
    pub time: String,
    pub temperature_2m_max: String,
    pub temperature_2m_min: String,
    pub weather_code: String,
    pub apparent_temperature_min: String,
    pub apparent_temperature_max: String,
    pub wind_speed_10m_max: String,
    pub precipitation_probability_max: String,
}

const FORECAST_BASE_URL: &str = "https://api.open-meteo.com/v1/forecast";
const GEOCODING_BASE_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";
const FORECAST_DEFAULTS: ForecastDefaults = ForecastDefaults {
    timezone: "auto",
    forecast_days: 3,
    wind_speed_unit: "mph",
    temperature_unit: "fahrenheit",
    precipitation_unit: "inch",
};
const GEOCODING_DEFAULTS: GeocodingDefaults = GeocodingDefaults {
    count: 10,
    language: "en",
    format: "json",
};

pub async fn test_form_req_url() -> Result<GeocodingResponse> {
    let daily_opts: Vec<DailyOpts> = vec![
        DailyOpts::Temperature2mMax,
        DailyOpts::Temperature2mMin,
        DailyOpts::WindSpeed10mMax,
        DailyOpts::WeatherCode,
        DailyOpts::ApparentTemperatureMin,
        DailyOpts::ApparentTemperatureMax,
        DailyOpts::PrecipitationProbabilityMax,
    ];

    let hourly_opts: Vec<HourlyOpts> = vec![
        HourlyOpts::Temperature2m,
        HourlyOpts::WindSpeed10m,
        HourlyOpts::WeatherCode,
        HourlyOpts::ApparentTemperature,
        HourlyOpts::PrecipitationProbability,
        HourlyOpts::Precipitation,
    ];

    let sample_req: ForecastRequestData = ForecastRequestData {
        latitude: 30.57881,
        longitude: -97.85307,
        daily: daily_opts,
        hourly: hourly_opts,
        timezone: String::from(FORECAST_DEFAULTS.timezone),
        forecast_days: FORECAST_DEFAULTS.forecast_days,
        wind_speed_unit: String::from(FORECAST_DEFAULTS.wind_speed_unit),
        temperature_unit: String::from(FORECAST_DEFAULTS.temperature_unit),
        precipitation_unit: String::from(FORECAST_DEFAULTS.precipitation_unit),
    };

    let sample_geo_req: GeocodingRequestData = GeocodingRequestData {
        name: String::from("78641"),
        count: GEOCODING_DEFAULTS.count,
        language: String::from(GEOCODING_DEFAULTS.language),
        format: String::from(GEOCODING_DEFAULTS.format),
    };

    println!("{:#?}", send_forecast_get(&sample_req).await.unwrap());

    send_geocoding_get(&sample_geo_req).await
}

fn construct_forecast_url(data: &ForecastRequestData) -> String {
    let daily = data.daily.iter().map(DailyOpts::as_str).collect::<Vec<_>>();
    let hourly = data
        .hourly
        .iter()
        .map(HourlyOpts::as_str)
        .collect::<Vec<_>>();

    let url: String = format!(
        "{}?latitude={}&longitude={}&daily={}&hourly={}&timezone={}&forecast_days={}&wind_speed_unit={}&temperature_unit={}&precipitation_unit={}",
        FORECAST_BASE_URL,
        data.latitude,
        data.longitude,
        daily.join(","),
        hourly.join(","),
        data.timezone,
        data.forecast_days,
        data.wind_speed_unit,
        data.temperature_unit,
        data.precipitation_unit
    );

    url
}

fn construct_geocoding_url(data: &GeocodingRequestData) -> String {
    let url: String = format!(
        "{}?name={}&count={}&language={}&format={}",
        GEOCODING_BASE_URL, data.name, data.count, data.language, data.format,
    );

    url
}

async fn send_forecast_get(data: &ForecastRequestData) -> Result<ForecastResponse> {
    let url: String = construct_forecast_url(data);

    let res = get(url)
        .await?
        .error_for_status()?
        .json::<ForecastResponse>()
        .await?;

    Ok(res)
}

async fn send_geocoding_get(data: &GeocodingRequestData) -> Result<GeocodingResponse> {
    let url: String = construct_geocoding_url(data);

    let res = get(url)
        .await?
        .error_for_status()?
        .json::<GeocodingResponse>()
        .await?;

    Ok(res)
}
