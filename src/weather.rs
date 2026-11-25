/* sample req url:
https://api.open-meteo.com/v1/forecast
?latitude=30.57881
&longitude=-97.85307
&daily=
    temperature_2m_max,
    temperature_2m_min,
    weather_code,
    apparent_temperature_min,
    apparent_temperature_max,
    wind_speed_10m_max,
    precipitation_probability_max
&hourly=
    temperature_2m,
    apparent_temperature,
    precipitation_probability,
    precipitation,
    weather_code,
    wind_speed_10m
&timezone=auto
&forecast_days=3
&wind_speed_unit=mph
&temperature_unit=fahrenheit
&precipitation_unit=inch
*/

use serde::{Deserialize, Serialize};

struct Defaults {
    timezone: &'static str,
    forecast_days: i8,
    wind_speed_unit: &'static str,
    temperature_unit: &'static str,
    precipitation_unit: &'static str,
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
    pub name: String, // zip code or search content
    pub count: u8, // 10
    pub language: String, // en
    pub format: String, // json
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherRequestData {
    pub latitude: f64,
    pub longitude: f64,
    pub daily: Vec<DailyOpts>,
    pub hourly: Vec<HourlyOpts>,
    pub timezone: String,
    pub forecast_days: i8,
    pub wind_speed_unit: String,
    pub temperature_unit: String,
    pub precipitation_unit: String,
}

const FORECAST_BASE_URL: &str = "https://api.open-meteo.com/v1/forecast";
const GEOCODING_BASE_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";
const DEFAULTS: Defaults = Defaults {
    timezone: "auto",
    forecast_days: 3,
    wind_speed_unit: "mph",
    temperature_unit: "fahrenheit",
    precipitation_unit: "inch"
};

pub fn test_form_req_url() -> String {
    let daily_opts: Vec<DailyOpts> = vec![
        DailyOpts::Temperature2mMax,
        DailyOpts::Temperature2mMin,
        DailyOpts::WindSpeed10mMax,
        DailyOpts::WeatherCode,
        DailyOpts::ApparentTemperatureMin,
        DailyOpts::ApparentTemperatureMax,
        DailyOpts::PrecipitationProbabilityMax
    ];

    let hourly_opts: Vec<HourlyOpts> = vec![
        HourlyOpts::Temperature2m,
        HourlyOpts::WindSpeed10m,
        HourlyOpts::WeatherCode,
        HourlyOpts::ApparentTemperature,
        HourlyOpts::PrecipitationProbability,
        HourlyOpts::Precipitation,
    ];

    /*
    &timezone=auto
&forecast_days=3
&wind_speed_unit=mph
&temperature_unit=fahrenheit
&precipitation_unit=inch\
 */
    let sample_req: WeatherRequestData = WeatherRequestData {
        latitude: 30.57881,
        longitude: -97.85307,
        daily: daily_opts,
        hourly: hourly_opts,
        timezone: String::from(DEFAULTS.timezone),
        forecast_days: DEFAULTS.forecast_days,
        wind_speed_unit: String::from(DEFAULTS.wind_speed_unit),
        temperature_unit: String::from(DEFAULTS.temperature_unit),
        precipitation_unit: String::from(DEFAULTS.precipitation_unit)
    };

    construct_forecast_url(&sample_req)
}

/* sample req url:
https://api.open-meteo.com/v1/forecast
?latitude=30.57881
&longitude=-97.85307
&daily=
    temperature_2m_max,
    temperature_2m_min,
    weather_code,
    apparent_temperature_min,
    apparent_temperature_max,
    wind_speed_10m_max,
    precipitation_probability_max
&hourly=
    temperature_2m,
    apparent_temperature,
    precipitation_probability,
    precipitation,
    weather_code,
    wind_speed_10m
&timezone=auto
&forecast_days=3
&wind_speed_unit=mph
&temperature_unit=fahrenheit
&precipitation_unit=inch
*/

fn construct_forecast_url(data: &WeatherRequestData) -> String {
    let daily = data.daily.iter().map(DailyOpts::as_str).collect::<Vec<_>>();
    let hourly = data.hourly.iter().map(HourlyOpts::as_str).collect::<Vec<_>>();

    let url: String = format!(
        "{}?latitude={}&longitude={}&daily={}&hourly={}&timezone={}&forecast_days={}&wind_speed_unit={}&temperature_unit={}precipitation_unit={}
        ",
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