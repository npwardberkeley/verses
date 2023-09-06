use std::net::IpAddr;

use rand::Rng;
use rocket::config::Config;
use rocket::http::ContentType;

#[macro_use]
extern crate rocket;
mod verses;

#[get("/")]
fn index() -> (ContentType, String) {
    (ContentType::Text, "Hello world!".to_string())
}

#[get("/html")]
fn html() -> (ContentType, String) {
    let verse = verses::get_random_verse();
    let phrase = get_random_phrase(verse.text);

    let result = phrase + "<br><br>" + &verse.reference.to_string();

    (ContentType::HTML, result.to_string())
}

#[get("/json")]
fn json() -> (ContentType, String) {
    let verse = verses::get_random_verse();
    let phrase = get_random_phrase(verse.text);

    let result = format!(
        "{{\"phrase\": \"{}\", \"reference\": \"{}\"}}",
        phrase,
        verse.reference.to_string()
    );

    (ContentType::JSON, result.to_string())
}

fn get_random_phrase(input: &'static str) -> String {
    let mut rng = rand::thread_rng();
    let words = input.split_whitespace();
    let num_words = words.clone().count();
    let length = 3;
    let start = rng.gen_range(0..(num_words - length));
    let end = start + length;
    let mut phrase = String::new();
    for (i, word) in words.enumerate() {
        if i >= start && i < end {
            phrase.push_str(word);
            phrase.push(' ');
        }
    }
    phrase
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(
            rocket::Config::figment()
                .merge(("port", 3000))
                .merge(("address", "0.0.0.0".parse::<IpAddr>().unwrap())),
        )
        .mount("/", routes![index])
        .mount("/", routes![html])
        .mount("/", routes![json])
}
