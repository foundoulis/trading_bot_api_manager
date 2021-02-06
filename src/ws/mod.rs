
use futures::{StreamExt};
use coinbase_pro_rs::{WSFeed, CBError, WS_URL};
use coinbase_pro_rs::structs::wsfeed::*;

pub async fn connect_and_print(product_ids: &[&str]) {
    let stream = WSFeed::new(WS_URL,
        product_ids, &[ChannelType::Level2]);
        stream
        .take(10)
        .for_each(|msg: Result<Message, CBError>| async {
        match msg.unwrap() {
            Message::Heartbeat {sequence, last_trade_id, time, ..} => println!("{}: seq: {} id: {}",
                                                                               time, sequence, last_trade_id),
            Message::Error {message} => println!("Error: {}", message),
            Message::InternalError(_) => panic!("internal_error"),
            Message::Level2(l2) => {
                match l2 {
                    Level2::L2update {product_id, changes} => {
                        println!("Update Message {}: {:?}", product_id, changes);
                    },
                    Level2::Snapshot {product_id, bids, asks} => {
                        println!("Level2 book: {:?}", product_id);
                        println!("Bids: {:?}", bids);
                        println!("Asks: {:?}", asks);
                    }
                };
            },
            other => println!("MISSED ME: {:?}", other)
        }
    }).await;
}
