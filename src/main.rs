use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    count: u32,
    data: Vec<WeatherData>,
}

#[derive(Deserialize, Debug)]
struct WeatherData {
    app_temp: f32,
    aqi: u32,
    city_name: String,
    clouds: u32,
    country_code: String,
    datetime: String,
    dewpt: f32,
    dhi: u32,
    dni: u32,
    elev_angle: f32,
    ghi: u32,
    gust: Option<f32>, // Nullable fields in JSON are represented as Option<T>
    h_angle: f32,
    lat: f64,
    lon: f64,
    ob_time: String,
    pod: String,
    precip: f32,
    pres: f32,
    rh: u32,
    slp: f32,
    snow: u32,
    solar_rad: u32,
    sources: Vec<String>, // JSON arrays map to Vec<T>
    state_code: String,
    station: String,
    sunrise: String,
    sunset: String,
    temp: f32,
    timezone: String,
    ts: u64,
    uv: u32,
    vis: u32,
    weather: WeatherDetails, // Nested object
    wind_cdir: String,
    wind_cdir_full: String,
    wind_dir: u32,
    wind_spd: f32,
}

#[derive(Deserialize, Debug)]
struct WeatherDetails {
    description: String,
    code: u32,
    icon: String,
}
#[tokio::main]

async fn main() -> Result<(), Error> {
    let url = "https://api.weatherbit.io/v2.0/current?lat=35.7796&lon=-78.6382&key=aa01545faaf74ee1a282ec933992bd15";
    let response = reqwest::get(url).await?;
    let weather: ApiResponse = response.json().await?;
    println!("{:#?}", weather);

    Ok(())
}
