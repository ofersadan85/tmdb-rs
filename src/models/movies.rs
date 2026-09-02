use super::common::Genre;
use serde::{Deserialize, Serialize};

/// A movie video entry attached through the append-to-response pattern.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Video {
    pub id: String,
    pub iso_639_1: String,
    pub key: String,
    pub name: String,
    pub site: String,
    pub size: u16,
    #[serde(rename = "type")]
    pub kind: String,
}

/// Movie details from the TMDB Movies chapter.
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Movie {
    pub id: u64,
    pub imdb_id: String,
    pub title: String,
    pub tagline: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String,
    pub runtime: u32,
    pub homepage: Option<String>,
    pub genres: Vec<Genre>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub budget: u64,
    pub adult: bool,
}

/// A movie entry returned by the search endpoints.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SearchMovie {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String,
    pub genre_ids: Vec<u16>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub adult: bool,
}

/// A movie item returned by the find endpoint.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FindMovie {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String,
    pub genre_ids: Vec<u16>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub adult: bool,
}

/// Movie search page returned by the TMDB search API.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SearchResult {
    pub page: u8,
    pub total_results: u8,
    pub total_pages: u8,
    pub results: Vec<SearchMovie>,
}

/// Find payload returned by the TMDB find endpoint.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FindResult {
    pub movie_results: Vec<FindMovie>,
}

/// Collection of movie details or metadata commonly attached to movie
/// responses.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct MovieCollection {
    pub id: u64,
    pub name: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}
