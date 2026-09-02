use serde::{Deserialize, Serialize};

/// Person model from the TMDB People chapter.
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Person {
    pub id: u64,
    pub adult: bool,
    pub also_known_as: Vec<String>,
    pub biography: String,
    pub birthday: Option<String>,
    pub deathday: Option<String>,
    pub gender: Option<u8>,
    pub homepage: Option<String>,
    pub imdb_id: Option<String>,
    pub known_for_department: String,
    pub name: String,
    pub place_of_birth: Option<String>,
    pub popularity: f64,
    pub profile_path: Option<String>,
}

/// A compact person result from search endpoints.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct PersonResult {
    pub id: u64,
    pub adult: bool,
    pub gender: Option<u8>,
    pub known_for_department: String,
    pub name: String,
    pub original_name: String,
    pub popularity: f64,
    pub profile_path: Option<String>,
}
