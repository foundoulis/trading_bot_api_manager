use reqwest;

pub struct CoinbaseClient {
    url: String,
    cli: reqwest::blocking::Client
}

impl CoinbaseClient {
    pub fn new() -> CoinbaseClient {
        CoinbaseClient {
            url: String::from("https://api.pro.coinbase.com/"),
            cli: reqwest::blocking::Client::new()
        }
    }

    pub fn ping_api(&self, route: &str) -> reqwest::Result<reqwest::blocking::Response> {
        self.cli.get(&format!("{}{}", self.url, route))
            .header("User-Agent", "foundoulis/1.0")
            .send()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = CoinbaseClient::new();
    let resp = client.ping_api("time").unwrap();
    println!("{:#?}", resp.text());
    Ok(())
}
