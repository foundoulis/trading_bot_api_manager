
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

    fn internal_request(&self, route: &str) -> reqwest::Result<reqwest::blocking::Response> {
        self.cli.get(&format!("{}{}", self.url, route))
            .header("User-Agent", "foundoulis/1.0")
            .send()
    }

    pub fn get_server_time(&self) -> reqwest::Result<reqwest::blocking::Response> {
        self.internal_request("time")
    }
}

pub fn coinbase_connect_get_time() {
    let client = CoinbaseClient::new();
    let resp = client.get_server_time().unwrap();
    println!("{:#?}", resp.text());
}
