use rand::prelude::*;
use serde::Deserialize;
use std::error::Error;
use crate::configuration::Config;

#[derive(Deserialize)]
pub struct Data {
    pub anime: String,
    pub character: String,
    pub quote: String,
}

impl Data {
    pub async fn build(config: Config) -> Result<Self, Box<dyn Error>> {
        let anime = get_title(config.anime.inner())?;
        let data: Self = serde_json::from_str(
            get_quote(anime)
            .await?
            .as_str()
        )?;

        Ok(data)
    }
}

fn get_title(titles: Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut rng = thread_rng();
    match titles.into_iter().choose(&mut rng) {
        Some(anime) => Ok(anime),
        None => return Err("No anime found.".into()),
    }
}

async fn get_quote(s: String) -> Result<String, Box<dyn Error>> {
    let data = reqwest::get(format!("https://animechan.vercel.app/api/random/anime?title={}", s))
        .await?
        .text()
        .await?;

    Ok(data)
}
