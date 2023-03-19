use scraper::Html;

fn main() ->  Result<(), Box<dyn std::error::Error>> {
    let url = "https://google.com/search";
    let query = "rust book";
    let params = [
        ("q", query)
    ];
    let url = reqwest::Url::parse_with_params(url, &params)?;
    let res = reqwest::blocking::get(url).unwrap().text().unwrap();

    // println!("Rust book search: = {:#?}", res);

    let document = Html::parse_document(&res);
    // println!("{:#?}", document);

    let link_selector = scraper::Selector::parse("div.yuRUbf>a").unwrap();
    // println!("{:#?}", link_selector);

    let links = document.select(&link_selector).map(|x| x.inner_html());
    println!("{:#?}", links);

    links
        .zip(1..3)
        .for_each(|(item, number)| println!("{}. {}", number, item));

    Ok(())
}
