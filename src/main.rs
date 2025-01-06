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
    confidence: u8,
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
}

#[derive(Deserialize, Debug)]
struct WeatherDetails {
    description: String,
    code: u32,
    icon: String,
}
#[tokio::main]

async fn main() -> Result<(), Error> {
    //   geol().await;
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
            output(weather);
        }
        Err(e) => {
            println!("teh error{}", e);
        }
    };
    Ok(())
}
fn output(data: ApiResponse) {
    for t in &data.data {
        println!("{:?}", t.weather.description);
        println!("Air quality: {}", t.aqi);
        println!("Temperate: {} celcius", t.temp);
        println!("UV: {} celcius", t.uv);
    }
}
fn input() -> String {
    loop {
        let mut input = String::new();
        println!("Enter the city/place name: ");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let city = input.trim();

        if !city.is_empty() && city.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
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
    loop {
        let query = input();
        let url = format!(
            "https://api.opencagedata.com/geocode/v1/json?q={}&key={}&no_annotations=1&limit=1",
            query, api_key
        );
        println!("{}", url);
        let resp = reqwest::get(url).await?;
        let data: OpenCageResponse = resp.json().await?;
        // Direct access using indexing

        if data.results.is_empty() {
            println!("its empty/not accurate");
            continue;
        } else {
            let confidence = data.results[0].confidence;
            println!("confi {}", confidence);
            if confidence > 1 {
                for la in data.results.iter() {
                    lat = la.geometry.lat;
                    long = la.geometry.lng;
                }
                break;
            }
            continue;
        }
    }
    return Ok((lat, long));
}

async fn geol() {
    let oc = Opencage::new("4bb89ad47e0b42bda10743925cf19e74".to_string());
    let address = input();

    let res: Vec<Point<f64>> = oc.forward(&address).unwrap();
    let first_result = &res[0];

    println!(
        " thsi is from geol {longitude}, {latitude}",
        longitude = first_result.x(),
        latitude = first_result.y()
    );
    // 11.5761796, 48.1599218
}
