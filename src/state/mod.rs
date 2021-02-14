
use coinbase_pro_rs::structs::wsfeed::Level2;
use coinbase_pro_rs::structs::reqs::OrderSide;
use num_traits::cast::FromPrimitive;
use num_traits::Zero;
use ordered_float::OrderedFloat;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
struct OrderBook {
    bids: BTreeMap<OrderedFloat<f64>, f64>,
    asks: BTreeMap<OrderedFloat<f64>, f64>,
    last_price: Option<f64>
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_price: None
        }
    }
    pub fn insert(&mut self, side: OrderSide, price: f64, size: f64) -> Option<f64> {
        match side {
            OrderSide::Buy => { self.insert_bid(price, size) }
            OrderSide::Sell => { self.insert_ask(price, size) }
        }
    }
    pub fn insert_bid(&mut self, price: f64, size: f64) -> Option<f64> {
        let key = OrderedFloat::from_f64(price).unwrap();
        if size == f64::zero() {
            self.bids.remove(&key)
        } else {
            self.bids.insert(key, size)
        }
    }
    pub fn insert_ask(&mut self, price: f64, size: f64) -> Option<f64> {
        let key = OrderedFloat::from_f64(price).unwrap();
        if size == f64::zero() {
            self.asks.remove(&key)
        } else {
            self.asks.insert(key, size)
        }
    }
    pub fn set_price(&mut self, price: f64) {
        self.last_price = Some(price);
    }
}

impl std::fmt::Display for OrderBook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (price, volume) in self.bids.iter().rev().take(5).rev() {
            writeln!(f, "BID: {:.2}-{:.1}", price.0, volume)?;
        }
        writeln!(f, "Last {}", self.last_price.unwrap_or(-1.0))?;
        for (price, volume) in self.asks.iter().take(5) {
            writeln!(f, "ASK: {:.2}-{:.1}", price.0, volume)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Markets(HashMap<String, OrderBook>);

impl Markets {
    pub fn new() -> Markets {
        Markets(HashMap::new())
    }
    
    fn get_mut(&mut self) -> &mut HashMap<String, OrderBook> {
        &mut self.0
    }

    fn get(&self) -> &HashMap<String, OrderBook> {
        & self.0
    }

    fn get_str_mut(&mut self, s: String) -> &mut OrderBook {
        if !self.get().contains_key(&s) {
            self.get_mut().insert(s.clone(), OrderBook::new());
        }
        self.get_mut().get_mut(&s).unwrap()
    }
    fn get_str(&self, s: String) -> Option<&OrderBook> {
        self.0.get(&s)
    }
    pub fn set_price(&mut self, (price, product_id): (f64, String)) {
        self.get_str_mut(product_id).set_price(price);
    }

    pub fn load_book(&mut self, books: Level2) {
        match books {
            Level2::Snapshot { product_id, bids, asks } => {
                let market = self.get_str_mut(product_id);

                for bid in bids {
                    let _old_size = market.insert_bid(bid.price, bid.size);
                }
                for ask in asks {
                    let _old_size = market.insert_ask(ask.price, ask.size);
                }
            },
            Level2::L2update { product_id, changes } => {
                let market = self.get_str_mut(product_id);
                for change in changes {
                    match change.side {
                        OrderSide::Buy => { market.insert_bid(change.price, change.size); }
                        OrderSide::Sell => { market.insert_ask(change.price, change.size); }
                    };
                }
            }
        };
    }
}

impl std::fmt::Display for Markets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for market in self.get() {
            writeln!(f, "{}\n{}", market.0, market.1)?;
        })
    }
}

