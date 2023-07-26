use axum::async_trait;

pub mod rarbg;

#[async_trait]
pub trait Database {
    async fn search_tvdb(id: i64) -> anyhow::Result<String>;
    async fn search_text(query: &str) -> anyhow::Result<String>;
}
