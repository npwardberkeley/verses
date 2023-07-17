use rocket::http::ContentType;
use rand::Rng;

#[macro_use] extern crate rocket;
mod verses;

// #[get("/")]
// fn index() -> (ContentType, String) {
//     let verse = verses::get_random_verse();

//     let phrase = get_random_phrase(verse.text);

//     let result = phrase + "<br><br>" + &verse.reference.to_string();

//     (ContentType::HTML, result.to_string())
// }

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
            phrase.push_str(" ");
        }
    }
    phrase
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
