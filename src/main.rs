// Imports
use std::io::Write;
use std::error::Error;
use env_logger::Builder;
use std::{thread, time};
use reqwest::{header::AUTHORIZATION, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use log::{info, error, debug, LevelFilter};


// OpenWeatherMap API Key:
static OPEN_WEATHER_API: &str = "INSERT YOUR API KEY HERE";
static ZIP_CODE: &str = "INSERT YOUR ZIP CODE HERE";
static UNITS: &str = "imperial";
static OWM_BASE_URI: &str = "http://api.openweathermap.org/data/2.5/weather?";
// Elasticsearch Settings
static ELASTIC_URL: &str = "https://ELASTICSEARCH.URL.GOES.HERE/<INDEX NAME>/_doc?pipeline=<PIPELINE NAME>";
static ELASTIC_AUTH: &str = "BASE64 ELASTICSEARCH API KEY HERE";


// Weather Struct Returned Data
#[derive(Serialize, Deserialize, Debug)]
struct WeatherData {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i64,
    sys: Sys,
    timezone: i32,
    id: i64,
    name: String,
    cod: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: i32,
    id: i32,
    country: String,
    sunrise: i64,
    sunset: i64,
}

#[tokio::main]
async fn main() {
    let mut builder = Builder::from_default_env();
    builder
        .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
        .filter(None, LevelFilter::Info)
        .init();
    
    // Throw this into a loop at some point cause whynot
    // 900000 = 15 Mins
    let sleep_time = time::Duration::from_millis(900000);
    loop {
        info!("Getting Weather...");
        let data = get_weather().await.expect("Failed to get data");
        info!("Posting Data to Elastic...");
        post_weather(data).await;
        info!("Completed, going to sleep...");
        thread::sleep(sleep_time);
    };
}

async fn post_weather(data: WeatherData) {
    // Create the Reqwest client
    let client = reqwest::Client::new();

    let mut response = client 
        .post(ELASTIC_URL)
        .header(AUTHORIZATION, format!("ApiKey {}", ELASTIC_AUTH))
        .header(CONTENT_TYPE, "application/json")
        .json(&data)
        .send()
        .await
        .unwrap();

    debug!("{:?}", response);
    debug!("{:?}", response.chunk().await);

}

// Gets and returns the weather 
async fn get_weather() -> Result<WeatherData, Box<dyn Error>> {
    //-> (f64, f64, f64, i32, i64, i64, String, String, i32)
    // Create the URL for the GET request
    let client = reqwest::Client::new();
    let owm_url: String = format!("{}zip={}&APPID={}&units={}", OWM_BASE_URI, ZIP_CODE, OPEN_WEATHER_API, UNITS);
    // Make the GET request against Open Weather Map
    let response = client.get(owm_url)
        .send()
        .await
        .unwrap();

    // Check to see if we successfully returned data, or not
    match response.status() {
        reqwest::StatusCode::OK => {
            info!("Success: {:?}", response.status());
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            error!("Unauthorized: {:?}", response.status());
        },
        _ => {
            error!("HTTP Status Code: {:?}", response.status());
        }
    }

    let data = response.json::<WeatherData>().await?;
    Ok(data)

}