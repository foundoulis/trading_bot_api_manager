
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use trading_bot_api_manager::cb_client::coinbase_api_client::CoinbaseClient;
use trading_bot_api_manager::cb_client::coinbase_ws_client::connect_and_print;
use trading_bot_api_manager::state::Markets;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let markets = Arc::new(Mutex::new(Markets::new()));
    
    let rt  = Runtime::new()?;
    rt.block_on(async {
        let conn_to_ws = connect_and_print(&["ETH-USD"], markets);
        let client = CoinbaseClient::new();
        let conn_to_time = client.get_server_time();
        
        conn_to_time.await;
        conn_to_ws.await;
    });

    Ok(())
}
