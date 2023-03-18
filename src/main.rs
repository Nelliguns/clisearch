fn main() ->  Result<(), Box<dyn std::error::Error>> {
    let url = "https://google.com/search";
    let query = "rustbook";
    let params = [
        ("q", query)
    ];
    let url = reqwest::Url::parse_with_params(url, &params)?;
    let res = reqwest::blocking::get(url)?;

    println!("Rust book search: = {:#?}", res.text().unwrap());

    Ok(())
}
