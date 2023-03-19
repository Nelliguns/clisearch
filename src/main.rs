use scraper::Html;
use std::env;

fn main() ->  Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let url = "https://google.com/search";
    let query = &args[1];
    let params = [
        ("q", query)
    ];
    let url = reqwest::Url::parse_with_params(url, &params)?;
    let res = reqwest::blocking::get(url).unwrap().text().unwrap();

    let document = Html::parse_document(&res);

    let link_selector = scraper::Selector::parse("a").unwrap();

    for element in document.select(&link_selector) {
        if let Some(link) = element.value().attr("href") {
            if link.starts_with("/url?q=") {
                let clean_link = &link[7..].split('&').next().unwrap();
                println!("{}", clean_link);
            }
        }
    }

    Ok(())
}
