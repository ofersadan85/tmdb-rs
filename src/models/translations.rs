use serde::{Deserialize, Serialize};

/// A translation payload returned by TMDB translation endpoints.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Translation {
    pub iso_3166_1: String,
    pub iso_639_1: String,
    pub name: String,
    pub english_name: String,
    pub data: TranslationData,
}

/// Translation content payload.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TranslationData {
    pub title: Option<String>,
    pub overview: Option<String>,
    pub homepage: Option<String>,
    pub tagline: Option<String>,
    pub runtime: Option<u32>,
    pub name: Option<String>,
}

/// Translation listing response.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TranslationList {
    pub id: u64,
    pub translations: Vec<Translation>,
}
