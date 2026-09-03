use serde::{Deserialize, Serialize};

/// Movie details from the TMDB Movies chapter.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Movie {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    #[serde(rename = "belongs_to_collection")]
    pub collection: Option<super::Collection>,
    pub budget: u64,
    pub genres: Vec<super::Genre>,
    pub homepage: Option<String>,
    pub id: u64,
    pub imdb_id: String,
    pub origin_country: Vec<String>,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub production_companies: Vec<super::Company>,
    pub production_countries: Vec<super::Country>,
    pub release_date: String,
    pub revenue: u64,
    pub runtime: u64,
    pub spoken_languages: Vec<super::Language>,
    pub status: String,
    pub tagline: String,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: u64,
}
