use serde::{Deserialize, Serialize};

/// Genre list payload from the TMDB Genres chapter.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GenreList {
    pub genres: Vec<crate::models::common::Genre>,
}
