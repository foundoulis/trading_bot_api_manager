use reqwest::{
    Error,
    Response
};
use std::future::Future;

pub struct CoinbaseClient {
    url: String,
    cli: reqwest::Client
}

impl CoinbaseClient {
    pub fn new() -> CoinbaseClient {
        CoinbaseClient {
            url: String::from("https://api.pro.coinbase.com/"),
            cli: reqwest::Client::new()
        }
    }

    async fn internal_request(&self, route: &str) -> Result<String, Error> {
        self.cli.get(&format!("{}{}", self.url, route))
            .header("User-Agent", "foundoulis/1.0")
            .send()
            .await?
            .text()
            .await
    }

    pub async fn get_server_time(&self) {
        match self.internal_request("time").await {
            Ok(o) => {
                println!("Ok ({})", o);
            },
            Err(e) => {
                println!("Err ({})", e);
            }
        };
    }
}
