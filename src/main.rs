use dotenv::dotenv;
use reqwest::blocking::{Client, ClientBuilder};
use serde_json::{Result, Value};
fn main() {
    dotenv().ok();
    let api_url = std::env::var("OPEN_WEATHER_API_URL").unwrap();
    let api_key = std::env::var("OPEN_WEATHER_API_KEY").unwrap();
    let lat = std::env::var("DEFAULT_LAT").unwrap();
    let lon = std::env::var("DEFAULT_LON").unwrap();
    let url = format!(
        "{}/weather?lat={}&lon={}&appid={}&units=imperial",
        api_url, lat, lon, api_key
    );
    let client = Client::new();
    let res = client.get(url).send();
    let raw_body = res.unwrap().text().unwrap();
    let body: Value = serde_json::from_str(&raw_body).unwrap();
    if let Some(weather) = body["weather"].as_array() {
        for info in weather {
            println!("Some weather {}", info["description"].as_str().unwrap());
        }
    }
}
