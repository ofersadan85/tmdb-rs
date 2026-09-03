//! Public TMDB domain models
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[expect(dead_code)]
mod account;
#[expect(dead_code)]
mod auth;
#[expect(dead_code)]
mod certifications;
#[expect(dead_code)]
mod changes;
#[expect(dead_code)]
mod discover;
#[expect(dead_code)]
mod favorites;
#[expect(dead_code)]
mod lists;
mod movies;
pub use movies::Movie;
mod people;
pub use people::Person;
#[expect(dead_code)]
mod reviews;
mod search;
pub use search::{ExternalSourceId, FindResponse, MultiSearch, SearchResults, Searchable};
#[expect(dead_code)]
mod session;
#[expect(dead_code)]
mod translations;
#[expect(dead_code)]
mod trending;
mod tv;
pub use tv::{Episode, Season, Tv};
#[expect(dead_code)]
mod watchlist;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

/// A simple image payload used by media endpoints.
#[expect(dead_code)]
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

/// A country object used in production and release information.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Country {
    pub iso_3166_1: String,
    pub name: String,
}

/// A language object used in production and release information.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Language {
    pub english_name: String,
    pub iso_639_1: String,
    pub name: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Keyword {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct Company {
    pub description: Option<String>,
    pub headquarters: Option<String>,
    pub homepage: Option<String>,
    pub id: u32,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: Option<String>,
    #[serde(rename = "parent_company")]
    pub parent: Option<Box<Self>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Collection {
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}

/// A video entry attached through the append-to-response pattern.
#[expect(dead_code)]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Video {
    pub iso_639_1: String,
    pub iso_3166_1: String,
    pub name: String,
    pub key: String,
    pub site: String,
    pub size: u16,
    #[serde(rename = "type")]
    pub kind: String,
    pub official: bool,
    pub published_at: String,
    pub id: String,
}
