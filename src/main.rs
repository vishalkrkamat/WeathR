use reqwest::Error;
use serde_json::Value;
use std::io;
struct Weather {
    temperature: String,
    weatehr: String,
    aqi: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // The URL of the API endpoint
    let url = "http://api.weatherapi.com/v1/current.json?key=3295f8e0c45248f69e963550243112&q=shillong&aqi=no";

    // Send an HTTP GET request
    let response = reqwest::get(url).await?;

    // Parse the response body as JSON
    let json: serde_json::Value = response.json().await?;

    // Print the JSON data
    println!("Response: {:#?}", json);

    Ok(())
}
