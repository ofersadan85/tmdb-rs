use serde::{Deserialize, Serialize};

/// A network attachment for a TV series.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Network {
    pub headquarters: Option<String>,
    pub homepage: Option<String>,
    pub id: u64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: String,
}

/// The final episode data returned in a TV details payload.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Episode {
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub vote_average: f64,
    pub vote_count: u64,
    pub air_date: String,
    #[serde(rename = "episode_number")]
    pub number: u32,
    pub production_code: Option<String>,
    pub runtime: u32,
    pub season_number: u32,
    pub show_id: u32,
    pub still_path: Option<String>,
}

/// TV season metadata.
#[derive(Debug, Deserialize, Serialize)]
pub struct Season {
    pub air_date: Option<String>,
    pub episode_count: u32,
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub poster_path: Option<String>,
    #[serde(rename = "season_number")]
    pub number: u32,
    pub vote_average: f64,
}

/// TV details from the TMDB TV chapter.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Series {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub created_by: Vec<super::people::Person>,
    pub episode_run_time: Vec<u64>,
    pub first_air_date: String,
    pub genres: Vec<super::Genre>,
    pub homepage: Option<String>,
    pub id: u64,
    pub in_production: bool,
    pub languages: Vec<super::Language>,
    pub last_air_date: String,
    pub last_episode_to_air: Option<Episode>,
    pub name: String,
    pub next_episode_to_air: Option<Episode>,
    pub networks: Vec<Network>,
    pub number_of_episodes: u32,
    pub number_of_seasons: u32,
    pub origin_country: Vec<String>,
    pub original_language: String,
    pub original_name: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub production_companies: Vec<super::Company>,
    pub production_countries: Vec<super::Country>,
    pub seasons: Vec<Season>,
    pub spoken_languages: Vec<super::Language>,
    pub status: String,
    pub tagline: String,
    #[serde(rename = "type")]
    pub show_type: String,
    pub vote_average: f64,
    pub vote_count: u64,
}

/// Alias for the [`Series`]
pub type Tv = Series;
