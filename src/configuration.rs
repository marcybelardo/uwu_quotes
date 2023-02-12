use toml::de::Error;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub anime: Anime
}

#[derive(Deserialize)]
pub struct Anime {
    pub titles: Vec<String>,
}

pub fn get_config() -> Result<Config, Error> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory.");
    let config_directory = base_path.join("configuration/anime.toml");

    let config_text = fs::read_to_string(config_directory).expect("Failed to load configuration.");

    let config: Config = toml::from_str(&config_text).expect("Failed to read TOML file.");

    Ok(config)
}