//! Public TMDB domain models
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};

mod account;
pub use account::{AccountDetails, AccountQuery, OnAccountLists, OnNonRatedAccountLists, SortBy};
#[expect(dead_code)]
mod auth;
#[expect(dead_code)]
mod certifications;
#[expect(dead_code)]
mod changes;
mod collections;
pub use collections::Collection;
mod companies;
pub use companies::{Company, Network};
#[expect(dead_code)]
mod discover;
mod lists;
pub use lists::ListInfo;
mod movies;
pub use movies::Movie;
mod people;
pub use people::Person;
mod rating;
pub use rating::Rating;
#[expect(dead_code)]
mod reviews;
mod search;
pub use search::{ExternalSourceId, FindResponse, MultiMedia, SearchResults, Searchable};
#[expect(dead_code)]
mod session;
#[expect(dead_code)]
mod translations;
mod trending;
mod tv;
pub use tv::{Episode, Season, Series, Tv};

#[derive(Debug, Deserialize, Serialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

/// A simple image payload used by media endpoints.
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    pub iso_3166_1: String,
    pub name: String,
}

/// A language object used in production and release information.
#[derive(Debug, Deserialize, Serialize)]
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

/// A video entry attached through the append-to-response pattern.
#[derive(Debug, Deserialize, Serialize)]
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

/// Media type used by the TMDB API when multiple types of media are involved.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Movie,
    Tv,
    Person,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct TmdbResponse {
    pub success: bool,
    pub status_code: u16,
    pub status_message: String,
}
