use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;

mod models;
use models::{ExternalSourceId, FindResponse, SearchResults, Searchable};

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
    /// This function reads the TMDB API token from the specified environment variable
    /// and initializes the client with the necessary authorization headers.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the client initialization fails for one of the following:
    /// - Failure to read the specified environment variable.
    /// - Invalid header value for the authorization header.
    pub fn from_env(env: &str) -> Result<Self, Error> {
        Self::from_token(&std::env::var(env)?)
    }

    /// Creates a new TMDB client instance.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the token is invalid as header value for the authorization header.
    pub fn from_token(token: &str) -> Result<Self, Error> {
        let bearer = format!("Bearer {token}");
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
    pub async fn search<T>(
        &self,
        params: T::Query,
    ) -> Result<SearchResults<T, T::Query>, reqwest::Error>
    where
        T: Searchable + DeserializeOwned + Send,
    {
        T::search(self, &self.base_url, params).await
    }

    /// Searches for items using a simple query string.
    /// Returns a [`SearchResults`] containing the search results.
    ///
    /// # Errors
    ///
    /// Returns [`reqwest::Error`] if the request to the TMDB API fails
    /// or if the response cannot be deserialized.
    pub async fn search_simple<T>(
        &self,
        query: impl Into<String>,
    ) -> Result<SearchResults<T, T::Query>, reqwest::Error>
    where
        T: Searchable + DeserializeOwned + Send,
    {
        T::search_simple(self, &self.base_url, query.into()).await
    }
}
