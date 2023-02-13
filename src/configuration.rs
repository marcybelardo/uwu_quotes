use std::fs;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
pub struct Config {
    pub anime: Anime,
}

#[derive(Deserialize)]
pub struct Anime {
    titles: Vec<String>,
}

impl Anime {
    pub fn inner(self) -> Vec<String> {
        self.titles
    }
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let base_path = std::env::current_dir()?;
    let config_directory = base_path.join("configuration/anime.toml");
    let config_text = fs::read_to_string(config_directory)?;
    let config: Config = toml::from_str(&config_text)?;

    Ok(config)
}
