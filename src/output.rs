use crate::request::Quote;
use uwuifier::uwuify_str_sse;

pub fn output(quote: Quote) {
    let uwu_quote = format!(
        "{}\n\n         ~*~ {}, {} ~*~",
        quote.quote,
        quote.character,
        quote.anime
    );

    println!(
        "{}", 
        uwuify_str_sse(uwu_quote.as_str())
    );
}