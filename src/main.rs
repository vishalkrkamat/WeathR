use geocoding::{Forward, Opencage, Point};
use reqwest::Error;
use serde::Deserialize;
use std::io::{self, Write};

#[derive(Deserialize, Debug)]
struct OpenCageResponse {
    results: Vec<Resultg>,
}

#[derive(Deserialize, Debug)]
struct Resultg {
    geometry: Geometry,
}

#[derive(Deserialize, Debug)]
struct Geometry {
    lat: f64,
    lng: f64,
}
#[derive(Deserialize, Debug)]

struct ApiResponse {
    count: u32,
    data: Vec<WeatherData>,
}

#[derive(Deserialize, Debug)]
struct WeatherData {
    aqi: u32,
    city_name: String,
    clouds: u32,
    country_code: String,
    datetime: String,
    dewpt: f32,
    gust: Option<f32>, // Nullable fields in JSON are represented as Option<T>
    precip: f32,
    pres: f32,
    rh: u32,
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
    match tes().await {
        Ok((lat, long)) => {
            println!("long {lat} and long {long}");
            let url = format!(
                "https://api.weatherbit.io/v2.0/current?lat={}&lon={}&key=aa01545faaf74ee1a282ec933992bd15",
                lat, long
            );
            println!("the url from weatherbit api:{}", url);
            let response = reqwest::get(url).await?;
            let weather: ApiResponse = response.json().await?;
            for weat in &weather.data {
                println!("{:?}", weat.weather.description);
            }
            println!("{:#?}", weather);
        }
        Err(e) => {
            println!("teh error{}", e);
        }
    };
    Ok(())
}
fn input() -> String {
    loop {
        let mut input = String::new();
        println!("Enter the city/place name: ");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let city = input.trim();

        if !city.is_empty() {
            return city.to_string(); // Return a valid string
        } else {
            println!("Invalid input. Please enter a non-empty string.");
            // No return here, loop continues
        }
    }
}
async fn tes() -> Result<(f64, f64), reqwest::Error> {
    let mut lat: f64 = 0.0;
    let mut long: f64 = 0.0;
    let api_key = "4bb89ad47e0b42bda10743925cf19e74";
    let query = input();
    let url = format!(
        "https://api.opencagedata.com/geocode/v1/json?q={}&key={}&no_annotations=1&limit=1",
        query, api_key
    );
    println!("{}", url);
    let resp = reqwest::get(url).await?;
    let data: OpenCageResponse = resp.json().await?;
    if data.results.is_empty() {
        println!("its empty ");
    } else {
        println!("NOT emtpy");
    }
    for la in data.results.iter() {
        lat = la.geometry.lat;
        long = la.geometry.lng;
    }
    return Ok((lat, long));
}
