use reqwest;
use serde_json::Value;
use std::env;

use crate::models::news::NewsItem;

pub async fn fetch_crypto_news(symbol: &str) -> Result<Vec<NewsItem>, reqwest::Error> {
    let api_key = env::var("CRYPTO_API_KEY").expect("API key not set");
    let url = format!(
        "https://cryptonews-api.com/api/v1/category?section=general&items=5&extra-fields=all&symbol={}&apikey={}",
        symbol, api_key
    );

    let resp = reqwest::get(&url).await?.json::<Value>().await?;

    let mut news_list = vec![];
    if let Some(articles) = resp.get("data").and_then(|d| d.as_array()) {
        for article in articles {
            let title = article.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let source = article.get("source_name").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let date = article.get("date").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let summary = article.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let url = article.get("news_url").and_then(|v| v.as_str()).unwrap_or("").to_string();

            news_list.push(NewsItem {
                title,
                source,
                date,
                summary,
                url,
            });
        }
    }

    Ok(news_list)
}
