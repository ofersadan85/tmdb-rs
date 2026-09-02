use serde::{Deserialize, Serialize};

/// A TMDB collection returned by the Collections chapter.
#[derive(Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct Collection {
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}

/// Collection details with nested parts.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct CollectionDetails {
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub parts: Vec<crate::models::movies::SearchMovie>,
}
