use serde::Serialize;

#[derive(Serialize)]
pub struct NewsItem {
    pub title: String,
    pub source: String,
    pub date: String,
    pub summary: String,
    pub url: String,
}
