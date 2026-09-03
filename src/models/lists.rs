use serde::{Deserialize, Serialize};

/// List metadata returned by the Lists chapter.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListInfo {
    pub created_by: String,
    pub description: String,
    pub favorite_count: u64,
    pub id: String,
    pub items: Vec<super::search::MultiSearch>,
    pub item_count: u64,
    pub iso_639_1: String,
    pub name: String,
    pub poster_path: Option<String>,
}
