use core::time;

use crate::{Movie, MultiMedia, Person, SearchResults, TmdbClient, Tv};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

/// Time window supported by the Trending endpoints.
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeWindow {
    Day,
    Week,
}

impl std::fmt::Display for TimeWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
        }
    }
}

#[async_trait::async_trait]
pub trait Trending: Sized + DeserializeOwned {
    const TRENDING_ENDPOINT: &'static str;

    async fn trending(
        client: &TmdbClient,
        time_window: TimeWindow,
        language: Option<String>,
    ) -> Result<SearchResults<Self, ()>, reqwest::Error> {
        let url = format!(
            "{}/trending/{}/{}",
            client.base_url,
            Self::TRENDING_ENDPOINT,
            time_window
        );
        let mut req = client.client.get(&url);
        if let Some(language) = language {
            req = req.query(&[("language", language)]);
        }
        req.send().await?.json().await
    }
}

impl Trending for MultiMedia {
    const TRENDING_ENDPOINT: &'static str = "all";
}

impl Trending for Movie {
    const TRENDING_ENDPOINT: &'static str = "movie";
}

impl Trending for Tv {
    const TRENDING_ENDPOINT: &'static str = "tv";
}

impl Trending for Person {
    const TRENDING_ENDPOINT: &'static str = "person";
}

impl TmdbClient {
    /// Fetches the trending items for the specified type and time window.
    /// 
    /// # Errors
    ///
    /// Returns [`reqwest::Error`] if the request fails.
    pub async fn trending<T: Trending + Send>(
        &self,
        time_window: TimeWindow,
        language: Option<String>,
    ) -> Result<SearchResults<T, ()>, reqwest::Error> {
        T::trending(self, time_window, language).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client() -> TmdbClient {
        dotenvy::dotenv().ok();
        TmdbClient::from_env("TMDB_TOKEN").unwrap()
    }

    async fn inner<T: Trending + Send>() -> SearchResults<T, ()> {
        let results = client().trending::<T>(TimeWindow::Day, None).await.unwrap();
        assert_eq!(results.page, 1);
        assert_eq!(
            results.results.len(),
            20,
            "TMDB search page size should be 20"
        );
        results
    }

    #[tokio::test]
    async fn all() {
        inner::<MultiMedia>().await;
    }

    #[tokio::test]
    async fn movies() {
        inner::<Movie>().await;
    }

    #[tokio::test]
    async fn tv() {
        inner::<Tv>().await;
    }

    #[tokio::test]
    async fn people() {
        inner::<Person>().await;
    }
}
