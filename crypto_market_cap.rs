use reqwest::Error;
use serde::{Deserialize, Serialize};
use tokio::task;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Coin {
    id: String,
    symbol: String,
    name: String,
    market_cap_usd: Option<f64>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let urls = vec![
        "https://api.coinlore.net/api/tickers/?start=0&limit=50",
        "https://api.coinlore.net/api/tickers/?start=50&limit=50",
    ];

    let mut tasks = vec![];

    for url in urls {
        tasks.push(task::spawn(async move {
            fetch_data(url).await
        }));
    }

    let mut all_coins = vec![];

    for task in tasks {
        let coins = task.await??;
        all_coins.extend(coins);
    }

    all_coins.sort_by(|a, b| b.market_cap_usd.partial_cmp(&a.market_cap_usd).unwrap());

    for coin in all_coins.iter().take(5) {
        println!(
            "ID: {}, Symbol: {}, Name: {}, Market Cap: {}",
            coin.id,
            coin.symbol,
            coin.name,
            coin.market_cap_usd.unwrap_or(0.0)
        );
    }

    Ok(())
}

async fn fetch_data(url: &str) -> Result<Vec<Coin>, Error> {
    let response = reqwest::get(url).await?.text().await?;
    let mut result: HashMap<String, Vec<Coin>> = serde_json::from_str(&response)?;
    Ok(result.remove("data").unwrap_or_default())
}
