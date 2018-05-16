#[macro_use]
extern crate serde_derive;

mod models;
mod scraper;
mod serializer;

use std::process;
use std::string::String;
use std::vec::Vec;

fn main() {
    let url = String::from("https://www.rust-lang.org/en-US/");
    let mut response = match scraper::request(url) {
        Ok(res) => res,
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    };

    let selected_langs: Vec<String> = scraper::select_languages(&mut response);
    let data = models::country::CountryData::create(&selected_langs);
    let json_serialize = serializer::serialize_from_countries(&data);

    println!("{}", json_serialize);
}
