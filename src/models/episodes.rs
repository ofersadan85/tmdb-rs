use serde::{Deserialize, Serialize};

/// TMDB episode details payload.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct EpisodeDetails {
    pub air_date: Option<String>,
    pub episode_number: u32,
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub production_code: Option<String>,
    pub runtime: Option<u32>,
    pub season_number: u32,
    pub show_id: u64,
    pub still_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: u64,
}

/// Episode image payload.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct EpisodeImages {
    pub id: u64,
    pub stills: Vec<crate::models::common::ImageInfo>,
}
