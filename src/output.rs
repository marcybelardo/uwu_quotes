use crate::request::Data;
use uwuifier::uwuify_str_sse;

pub fn output(data: Data) {
    let uwu_quote = format!(
        "{}\n\n         ~*~ {}, {} ~*~",
        data.quote,
        data.character,
        data.anime
    );

    println!(
        "{}", 
        uwuify_str_sse(uwu_quote.as_str())
    );
}