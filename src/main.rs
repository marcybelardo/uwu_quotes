use uwu_quote_bot::configuration::get_config;
use uwu_quote_bot::request::Data;
use uwu_quote_bot::output::output;

#[tokio::main]  
async fn main() {
    let config = match get_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Could not load config: {e}");
            std::process::exit(1);
        },
    };

    let data = match Data::build(config).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to get quote: {e}");
            std::process::exit(1);
        },
    };

    output(data);
}