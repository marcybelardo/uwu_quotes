use rand::prelude::*;
use serde::{Serialize, Deserialize};
use std::error::Error;
use uwuifier::uwuify_str_sse;

#[derive(Serialize, Deserialize)]
struct Quote {
    anime: String,
    character: String,
    quote: String,
}

impl Quote {
    async fn get_quote() -> Result<Self, Box<dyn Error>> {
        let animes = vec![
            "berserk",
            "evangelion",
            "serial%20experiments%20lain",
            "monster"
        ];

        let mut rng = thread_rng();
        let anime = animes.into_iter()
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

#[tokio::main]  
async fn main() {
    let quote = match Quote::get_quote().await {
        Ok(quote) => quote,
        Err(e) => {
            eprintln!("Failed to get quote: {:?}", e);
            std::process::exit(1);
        },
    };

    let uwu_quote = format!(
        "{}\n\n         ~*~ {}, {} ~*~",
        quote.quote,
        quote.character,
        quote.anime
    );

    let output = uwuify_str_sse(uwu_quote.as_str());

    println!("{}", output);
}