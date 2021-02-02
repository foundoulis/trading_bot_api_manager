use reqwest;

pub fn ping_api(route: &str) -> reqwest::Result<reqwest::blocking::Response> {
    let mut cb_url = String::from("https://api.pro.coinbase.com/");
    cb_url.push_str(route);
    reqwest::blocking::get(&cb_url)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = ping_api(&String::from("currencies")).unwrap();
    println!("{:#?}", resp.text());
    Ok(())
}
