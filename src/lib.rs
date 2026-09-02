use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

mod models;
use models::search::{ExternalSourceId, FindResponse, SearchQuery, SearchResults};

/// Error type for TMDB client operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Var(#[from] std::env::VarError),
    #[error(transparent)]
    HeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}

/// TMDB client for interacting with the TMDB API.
#[derive(Clone)]
pub struct TmdbClient {
    client: reqwest::Client,
    base_url: String,
}

impl TmdbClient {
    /// Creates a new TMDB client instance.
    ///
    /// This function reads the TMDB API token from the environment variable `TMDB_TOKEN`
    /// and initializes the client with the necessary authorization headers.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the client initialization fails for one of the following:
    /// - Failure to read the `TMDB_TOKEN` environment variable.
    /// - Invalid header value for the authorization header.
    pub fn new() -> Result<Self, Error> {
        dotenvy::dotenv().ok();
        let bearer = format!("Bearer {}", std::env::var("TMDB_TOKEN")?);
        let mut bearer = HeaderValue::from_str(&bearer)?;
        bearer.set_sensitive(true);
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, bearer);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Self {
            client,
            base_url: "https://api.themoviedb.org/3".to_string(),
        })
    }

    /// Finds items by their external ID using the specified external source.
    ///
    /// # Arguments
    ///
    /// `external_id` - The external ID of the item to find.
    /// `external_source` - The external source to use for the lookup.
    /// `language` - Optional language parameter for the response.
    ///
    /// # Returns
    ///
    /// A [`FindResponse`] containing the search results.
    ///
    /// # Errors
    ///
    /// Returns [`reqwest::Error`] if the request to the TMDB API fails
    /// or if the response cannot be deserialized.
    pub async fn find_by_external_id(
        &self,
        external_id: &str,
        external_source: ExternalSourceId,
        language: Option<&str>,
    ) -> Result<FindResponse, reqwest::Error> {
        let url = format!("{}/find/{external_id}", self.base_url);
        let mut req = self
            .client
            .get(&url)
            .query(&[("external_source", external_source)]);
        if let Some(language) = language {
            req = req.query(&[("language", language)]);
        }
        req.send().await?.error_for_status()?.json().await
    }

    /// Searches for items using the specified search query parameters.
    /// Returns a [`SearchResults`] containing the search results.
    ///
    /// # Errors
    ///
    /// Returns [`reqwest::Error`] if the request to the TMDB API fails
    /// or if the response cannot be deserialized.
    pub async fn search(
        &self,
        params: impl Into<SearchQuery<'_>>,
    ) -> Result<SearchResults, reqwest::Error> {
        let params = params.into();
        self.client
            .get(format!("{}/{}", self.base_url, params.uri()))
            .query(&params)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::search::*;

    #[tokio::test]
    async fn external_id() {
        let response = TmdbClient::new()
            .unwrap()
            .find_by_external_id("tt33764258", ExternalSourceId::Imdb, None)
            .await
            .unwrap();
        assert!(!response.movies.is_empty(), "{response:#?}");
        assert_eq!(response.movies[0].title, "The Odyssey");
    }

    #[tokio::test]
    async fn search_movie() {
        let response = TmdbClient::new()
            .unwrap()
            .search(SearchQuery::simple_movie("Inception"))
            .await
            .unwrap();

        assert!(!response.results.is_empty(), "{response:#?}");
    }

    #[tokio::test]
    async fn search_tv() {
        let response = TmdbClient::new()
            .unwrap()
            .search(SearchQuery::simple_tv("Dark"))
            .await
            .unwrap();

        assert!(!response.results.is_empty(), "{response:#?}");
    }

    #[tokio::test]
    async fn search_person() {
        let response = TmdbClient::new()
            .unwrap()
            .search(SearchQuery::simple_person("Keanu Reeves"))
            .await
            .unwrap();

        assert!(!response.results.is_empty(), "{response:#?}");
    }

    #[tokio::test]
    async fn search_collection() {
        let response = TmdbClient::new()
            .unwrap()
            .search(SearchQuery::simple_collection("Star Wars"))
            .await
            .unwrap();

        assert!(!response.results.is_empty(), "{response:#?}");
    }

    #[tokio::test]
    async fn search_company() {
        let response = TmdbClient::new()
            .unwrap()
            .search(SearchQuery::simple_company("Pixar"))
            .await
            .unwrap();

        assert!(!response.results.is_empty(), "{response:#?}");
    }

    #[tokio::test]
    async fn search_keyword() {
        let response = TmdbClient::new()
            .unwrap()
            .search(SearchQuery::simple_keyword("space"))
            .await
            .unwrap();

        assert!(!response.results.is_empty(), "{response:#?}");
    }

    #[tokio::test]
    async fn search_multi() {
        let response = TmdbClient::new()
            .unwrap()
            .search(SearchQuery::simple_multi("Avatar"))
            .await
            .unwrap();

        assert!(!response.results.is_empty(), "{response:#?}");
    }
}
