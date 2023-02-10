use rand::prelude::*;
use serde::{Serialize, Deserialize};
use std::error::Error;
use crate::configuration::Settings;

#[derive(Serialize, Deserialize)]
pub struct Quote {
    pub anime: String,
    pub character: String,
    pub quote: String,
}

impl Quote {
    pub async fn get_quote(settings: Settings) -> Result<Self, Box<dyn Error>> {
        let mut rng = thread_rng();
        let anime = settings.animes.into_iter()
            .choose(&mut rng)
            .unwrap();

        let data = reqwest::get(format!("https://animechan.vercel.app/api/random/anime?title={}", anime))
            .await?
            .text()
            .await?;
    
        let quote: Self = serde_json::from_str(&data)?;

        Ok(quote)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn check_if_requests_go_through() {
        let settings = Settings {
            animes: vec![
            "berserk".to_string(),
            "evangelion".to_string(),
            "monster".to_string(),
            ],
        };

        assert!(Quote::get_quote(settings).await.is_ok());
    }
}