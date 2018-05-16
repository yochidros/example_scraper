extern crate serde;
extern crate serde_json;

use super::models::country;

pub fn serialize_from_countries<'a>(object: &'a country::CountryData) -> String {
    serde_json::to_string(&object).unwrap()
}
