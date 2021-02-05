use futures::{StreamExt};
use coinbase_pro_rs::{WSFeed, CBError, WS_SANDBOX_URL};
use coinbase_pro_rs::structs::wsfeed::*;
use tokio::runtime::Runtime;

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

fn _coinbase_test() {
    let client = CoinbaseClient::new();
    let resp = client.get_server_time().unwrap();
    println!("{:#?}", resp.text());
}

async fn _coinbase_ws_test() {
    let stream = WSFeed::new(WS_SANDBOX_URL,
        &["BTC-USD"], &[ChannelType::Heartbeat]);

    stream
        .take(10)
        .for_each(|msg: Result<Message, CBError>| async {
        match msg.unwrap() {
            Message::Heartbeat {sequence, last_trade_id, time, ..} => println!("{}: seq:{} id{}",
                                                                               time, sequence, last_trade_id),
            Message::Error {message} => println!("Error: {}", message),
            Message::InternalError(_) => panic!("internal_error"),
            other => println!("{:?}", other)
        }
    }).await;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    _coinbase_test();

    let rt  = Runtime::new()?;
    rt.block_on(async {
        _coinbase_ws_test().await;
    });

    Ok(())
}
