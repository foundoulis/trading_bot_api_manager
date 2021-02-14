
use futures::{StreamExt};
use coinbase_pro_rs::{WSFeed, CBError, WS_URL};
use coinbase_pro_rs::structs::wsfeed::*;
use crate::state::Markets;
use std::sync::{Arc, Mutex};

pub async fn connect_and_print(product_ids: &[&str]) {
    let markets = Arc::new(Mutex::new(Markets::new()));
    let stream = WSFeed::new(WS_URL, product_ids, &[ChannelType::Level2, ChannelType::Ticker]);
    stream
        .take(100)
        .for_each_concurrent(None, |msg: Result<Message, CBError>| async {
        match msg.unwrap() {
            Message::Heartbeat {sequence, last_trade_id, time, ..} => println!("{}: seq: {} id: {}",
                                                                                time, sequence, last_trade_id),
            Message::Error {message} => println!("Error: {}", message),
            Message::InternalError(_) => panic!("internal_error"),
            Message::Level2(l2) => {
                markets.lock().unwrap().load_book(l2);
            },
            Message::Ticker(t) => {
                markets.lock().unwrap().set_price(match t {
                    Ticker::Full {product_id, price, ..} => { (price, product_id) },
                    Ticker::Empty {product_id, price, ..} => { (price, product_id) },
                });
            },
            Message::Subscriptions { channels } => {},
            Message::Match(_) => {},
            Message::Full(_) => {}
        };
        println!("{}", markets.lock().unwrap());
    }).await;
}
