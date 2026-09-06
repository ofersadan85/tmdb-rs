use serde::{Deserialize, Serialize};

#[derive(Debug, Default, serde_repr::Deserialize_repr, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum Gender {
    #[default]
    Unknown = 0,
    Male = 1,
    Female = 2,
    Other = 3,
}

/// Person model from the TMDB People chapter.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Person {
    pub adult: bool,
    pub also_known_as: Vec<String>,
    pub biography: Option<String>,
    pub birthday: Option<String>,
    pub deathday: Option<String>,
    pub gender: Gender,
    pub homepage: Option<String>,
    pub id: u64,
    pub imdb_id: Option<String>,
    pub known_for_department: Option<String>,
    pub name: String,
    pub place_of_birth: Option<String>,
    pub popularity: f64,
    pub profile_path: Option<String>,
    pub known_for: Vec<crate::MultiMedia>,
}
