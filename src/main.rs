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
        let data = reqwest::get("https://animechan.vercel.app/api/random/anime?title=berserk")
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

    let uwu_quote = uwuify_str_sse(quote.quote.as_str());

    println!("{}", uwu_quote);
}
