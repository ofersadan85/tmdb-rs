use crate::MultiMedia;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Collection {
    pub id: u64,
    pub name: String,
    pub original_language: String,
    pub original_name: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub parts: Vec<MultiMedia>,
}

impl Collection {
    /// Retrieves the details of a collection by its ID.
    #[doc = crate::tmdb_api_error_docs!()]
    pub async fn details(
        client: &crate::TmdbClient,
        id: u64,
        language: Option<&str>,
    ) -> Result<Self, reqwest::Error> {
        let url = format!("{}/collection/{id}", client.base_url);
        let mut req = client.client.get(&url);
        if let Some(language) = language {
            req = req.query(&[("language", language)]);
        }
        req.send().await?.json().await
    }
}
