use serde::{Deserialize, Serialize};

/// Time window supported by the Trending endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum TimeWindow {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
}

/// Trending item payload.
#[derive(Debug, Deserialize, Serialize)]
pub struct TrendingItem {
    pub adult: Option<bool>,
    pub backdrop_path: Option<String>,
    pub first_air_date: Option<String>,
    pub genre_ids: Vec<u64>,
    pub id: u64,
    pub media_type: super::MediaType,
    pub name: Option<String>,
    pub origin_country: Vec<String>,
    pub original_language: String,
    pub original_name: Option<String>,
    pub original_title: Option<String>,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub release_date: Option<String>,
    pub title: Option<String>,
    pub video: Option<bool>,
    pub vote_average: f64,
    pub vote_count: u64,
}
