extern crate reqwest;
extern crate scraper;

pub fn select_languages(response: &mut reqwest::Response) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();

    let parse_body = scraper::Html::parse_fragment(&response.text().unwrap());
    let footer_selector = scraper::Selector::parse("footer").unwrap();
    let a_selector = scraper::Selector::parse("a").unwrap();
    let footer = parse_body.select(&footer_selector).next().unwrap();
    for element in footer.select(&a_selector) {
        results.push(element.inner_html());
    }

    results
}

pub fn request(url: String) -> Result<reqwest::Response, String> {
    let request = reqwest::get(url.as_str());

    let response: reqwest::Response = match request {
        Ok(res) => {
            if check_status_code(&res) {
                res
            } else {
                return Err(String::from("error"));
            }
        }
        Err(error) => {
            println!("{}", error);
            return Err(error.to_string());
        }
    };

    Ok(response)
}

fn check_status_code<'a>(response: &'a reqwest::Response) -> bool {
    match response.status() {
        reqwest::StatusCode::Ok => true,
        _ => false,
    }
}
