use serde::{Deserialize, Serialize};

/// Discover request filters used by the TMDB Discover chapter.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DiscoverFilters {
    pub page: Option<u32>,
    pub with_genres: Option<String>,
    pub sort_by: Option<String>,
    pub language: Option<String>,
    pub region: Option<String>,
    pub vote_average_gte: Option<f64>,
    pub vote_average_lte: Option<f64>,
}
