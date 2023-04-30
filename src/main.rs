use reqwest;
use tokio;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct WeatherForecast {
    latitude: f64,
    longitude: f64,
    current_weather: WeatherData
}
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct WeatherData {
  temperature: f32,
}
    #[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=-36.85&longitude=174.76&current_weather=true";
    let response= reqwest::get(url).await?.text().await?;
    let weather_forecast: WeatherForecast = serde_json::from_str(&response).unwrap();

    println!("Current temperature is: {:?}", weather_forecast.current_weather.temperature);
    Ok(())
}

// https://api.open-meteo.com/v1/forecast?latitude=-36.85&longitude=174.76&current_weather=true