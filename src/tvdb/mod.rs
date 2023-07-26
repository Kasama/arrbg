use reqwest::{IntoUrl, Method};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Series {
    pub title: String,
    #[serde(rename = "imdbId")]
    pub imdb_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SonarrClient {
    api_key: Secret<String>,
    base_url: String,
    client: reqwest::Client,
}

const API_KEY_HEADER: &str = "X-Api-Key";

impl SonarrClient {
    pub fn new(base_url: String, api_key: String) -> anyhow::Result<Self> {
        Ok(Self {
            api_key: Secret::new(api_key),
            base_url,
            client: reqwest::Client::new(),
        })
    }

    fn request<U: IntoUrl>(&self, method: reqwest::Method, url: U) -> reqwest::RequestBuilder {
        self.client
            .request(method, url)
            .header(API_KEY_HEADER, self.api_key.expose_secret())
    }

    pub async fn get_series_metadata(&self, tvdb_id: &str) -> anyhow::Result<Vec<Series>> {
        let series = self
            .request(
                Method::GET,
                format!(
                    "{}/api/v3/series/lookup?term=tvdb:{}",
                    self.base_url, tvdb_id
                ),
            )
            .send()
            .await?
            .json::<Vec<Series>>()
            .await?;

        Ok(series)
    }
}
