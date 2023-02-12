use uwu_quote_bot::configuration::get_config;
use uwu_quote_bot::request::Quote;
use uwu_quote_bot::output::output;

#[tokio::main]  
async fn main() {
    let settings = get_config().expect("Could not read configuration.");
    let quote = match Quote::get_quote(settings).await {
        Ok(quote) => quote,
        Err(e) => {
            eprintln!("Failed to get quote: {:?}", e);
            std::process::exit(1);
        },
    };

    output(quote);
}