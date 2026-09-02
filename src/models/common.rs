use serde::{Deserialize, Serialize};

/// Shared TMDB response wrapper for a list of results.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Results<T> {
    pub results: Vec<T>,
}

/// Generic TMDB identifier model used across many endpoints.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

/// A common status object returned by TMDB APIs.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct StatusResponse {
    pub success: bool,
    pub status_code: u16,
    pub status_message: String,
}

/// A simple image payload used by media endpoints.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ImageInfo {
    pub aspect_ratio: f64,
    pub height: u32,
    pub iso_639_1: Option<String>,
    pub file_path: String,
    pub vote_average: Option<f64>,
    pub vote_count: u64,
    pub width: u32,
}

/// A network attachment for a TV series.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Network {
    pub id: u64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: String,
}
