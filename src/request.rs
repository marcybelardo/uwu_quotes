use rand::prelude::*;
use serde::Deserialize;
use std::error::Error;
use crate::configuration::Config;

#[derive(Deserialize)]
pub struct Quote {
    pub anime: String,
    pub character: String,
    pub quote: String,
}

impl Quote {
    pub async fn get_quote(config: Config) -> Result<Self, Box<dyn Error>> {
        let mut rng = thread_rng();
        let anime = match config.anime.titles.into_iter().choose(&mut rng) {
            Some(anime) => anime,
            None => return Err("No anime found.".into()),
        };

        let data = reqwest::get(format!("https://animechan.vercel.app/api/random/anime?title={}", anime))
            .await?
            .text()
            .await?;
    
        let quote: Self = serde_json::from_str(&data)?;

        Ok(quote)
    }
}
