use crate::*;

pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        Config {
            api_key: env::var("API_KEY").expect("API_KEY is not set in .env"),
        }
    }
}
