mod cli;
mod config;
mod utils;
use serde::{Deserialize, Serialize};
use std::env;

use crate::cli::Args;
use crate::config::Config;
use crate::utils::set_table_style;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Weather {
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Wind {
    speed: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = Args::new();
    let config = Config::new();

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        cmd.city, config.api_key
    );

    let response = reqwest::get(url).await;
    match response {
        Ok(res) => {
            let weather = res.json::<Weather>().await.unwrap();
            set_table_style(&weather);
            println!("Success")
        }
        Err(_) => println!("Failed"),
    }

    Ok(())
}
