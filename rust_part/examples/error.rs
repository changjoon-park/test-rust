use serde::Deserialize;
use serde_json::Result as JsonResult;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Config {
    database_url: String,
    server_port: u16,
    debug_mode: bool,
}

fn read_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_config(contents: &str) -> JsonResult<Config> {
    let config: Config = serde_json::from_str(contents)?;
    Ok(config)
}

fn load_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = read_file(file_path)?;
    let config = parse_config(&contents)?;
    Ok(config)
}

fn main() {
    let config_path = "config.json";
    match load_config(config_path) {
        Ok(config) => println!("Configuration loaded: {:?}", config),
        Err(e) => eprintln!("Failed to load configuration: {}", e),
    }
}
