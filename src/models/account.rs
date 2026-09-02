use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Gravatar {
    pub hash: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TmdbAvatar {
    pub avatar_path: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Avatar {
    pub gravatar: Option<Gravatar>,
    pub tmdb: Option<TmdbAvatar>,
}

/// Public account details on TMDB.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccountDetails {
    pub id: u64,
    pub name: String,
    pub username: String,
    #[serde(rename = "iso_639_1")]
    pub language: String,
    #[serde(rename = "iso_3166_1")]
    pub country: String,
    pub avatar: Option<Avatar>,
    pub include_adult: bool,
}
