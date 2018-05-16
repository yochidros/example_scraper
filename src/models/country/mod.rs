#[derive(Serialize, Debug)]
pub struct CountryData {
    countries: Vec<Country>,
}

#[derive(Serialize, Debug)]
struct Country {
    language: String,
}

impl CountryData {
    pub fn create<'a>(languages: &'a Vec<String>) -> CountryData {
        let mut countries: Vec<Country> = Vec::new();

        for lang in languages.iter() {
            let country = Country {
                language: lang.to_string(),
            };
            countries.push(country);
        }

        CountryData {
            countries: countries,
        }
    }
}
