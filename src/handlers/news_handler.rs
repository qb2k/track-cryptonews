use actix_web::{web, HttpResponse, Responder};
use crate::api::coingecko;
use tera::Tera;
use serde::{Deserialize, Serialize};
use tera::Context;
use reqwest;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NewsItem {
    title: String,
    url: String,
    source: String,
    published_on: i64,
}

async fn fetch_crypto_news(symbol: &str) -> Result<Vec<NewsItem>, reqwest::Error> {
    let url = format!("https://min-api.cryptocompare.com/data/v2/news/?categories={}", symbol);
    let response = reqwest::get(&url).await?;
    let news_data: serde_json::Value = response.json().await?;
    
    let news_items: Vec<NewsItem> = news_data["Data"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .take(4)
        .map(|item| NewsItem {
            title: item["title"].as_str().unwrap_or("").to_string(),
            url: item["url"].as_str().unwrap_or("").to_string(),
            source: item["source"].as_str().unwrap_or("").to_string(),
            published_on: item["published_on"].as_i64().unwrap_or(0),
        })
        .collect();

    Ok(news_items)
}

pub async fn search(
    form: web::Form<SearchQuery>,
    tmpl: web::Data<Tera>,
) -> impl Responder {
    let crypto_data = coingecko::fetch_coingecko_data(&form.symbol).await.unwrap_or(vec![]);
    let crypto_news = fetch_crypto_news(&form.symbol).await.unwrap_or(vec![]);

    let mut ctx = Context::new();
    ctx.insert("symbol", &form.symbol);
    ctx.insert("crypto_data", &crypto_data);
    ctx.insert("crypto_news", &crypto_news);

    let rendered = tmpl.render("result.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn get_news(symbol: web::Path<String>) -> impl Responder {
    match fetch_crypto_news(&symbol).await {
        Ok(news) => HttpResponse::Ok().json(news),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching news"),
    }
}
