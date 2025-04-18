use reqwest;
use serde_json::Value;
use crate::models::news::NewsItem;

pub async fn fetch_coingecko_data(coin: &str) -> Result<Vec<NewsItem>, reqwest::Error> {
    let coin_id = coin.to_lowercase();

    let desc_url = format!("https://api.coingecko.com/api/v3/coins/{}", coin_id);
    let desc_resp = reqwest::get(&desc_url).await?.json::<Value>().await;

    let price_url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        coin_id
    );
    let price_resp = reqwest::get(&price_url).await?.json::<Value>().await;

    let mut news_list = Vec::new();

    if let Ok(desc_json) = &desc_resp {
        if let Some(desc) = desc_json["description"]["en"].as_str() {
            news_list.push(NewsItem {
                title: format!("About {}", coin.to_uppercase()),
                source: "CoinGecko (Description)".to_string(),
                date: "N/A".to_string(),
                summary: desc.to_string(),
                url: desc_json["links"]["homepage"][0].as_str().unwrap_or("").to_string(),
            });
        }
    }

    if let Ok(price_json) = &price_resp {
        if let Some(price) = price_json[&coin_id]["usd"].as_f64() {
            news_list.push(NewsItem {
                title: format!("Current Price of {}", coin.to_uppercase()),
                source: "CoinGecko (Price)".to_string(),
                date: "N/A".to_string(),
                summary: format!("${:.2}", price),
                url: "https://www.coingecko.com".to_string(),
            });
        }
    }

    Ok(news_list)
}
