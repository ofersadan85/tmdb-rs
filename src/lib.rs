use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

mod models;
pub use models::*;

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
}
