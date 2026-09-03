use super::{
    common::{Genre, Network},
    companies::Company,
};
use serde::{Deserialize, Serialize};

/// The final episode data returned in a TV details payload.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Episode {
    pub air_date: String,
    #[serde(rename = "episode_number")]
    pub number: u32,
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub production_code: Option<String>,
    pub season_number: u32,
    pub still_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: u64,
}

/// TV season metadata.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Season {
    pub air_date: Option<String>,
    pub episode_count: u32,
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub poster_path: Option<String>,
    #[serde(rename = "season_number")]
    pub number: u32,
}

/// Cast entry for a TV credits response.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TVCast {
    pub id: u64,
    pub credit_id: String,
    pub character: String,
    pub gender: Option<u8>,
    pub name: String,
    pub profile_path: Option<String>,
    pub order: u32,
}

/// Creator data for a TV series.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TVCreator {
    pub id: u64,
    pub credit_id: String,
    pub name: String,
    pub gender: Option<u8>,
    pub profile_path: Option<String>,
}

/// TV details from the TMDB TV chapter.
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Tv {
    pub id: u64,
    pub backdrop_path: Option<String>,
    pub created_by: Vec<TVCreator>,
    pub episode_run_time: Vec<u64>,
    pub first_air_date: String,
    pub genres: Vec<Genre>,
    pub homepage: Option<String>,
    pub in_production: bool,
    pub languages: Vec<String>,
    pub last_air_date: String,
    pub last_episode_to_air: Option<Episode>,
    pub name: String,
    pub networks: Vec<Network>,
    pub number_of_episodes: u32,
    pub number_of_seasons: u32,
    pub origin_country: Vec<String>,
    pub original_language: String,
    pub original_name: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub production_companies: Vec<Company>,
    pub seasons: Vec<Season>,
    pub status: String,
    pub r#type: String,
    pub vote_average: f64,
    pub vote_count: u64,
}
