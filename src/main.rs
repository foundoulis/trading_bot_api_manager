
use tokio::runtime::Runtime;
use trading_bot_api_manager::ws::connect_and_print;
use trading_bot_api_manager::client::coinbase_connect_get_time;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    coinbase_connect_get_time();

    let rt  = Runtime::new()?;
    rt.block_on(async {
        connect_and_print(&["ETH-USD"]).await;
    });

    Ok(())
}
