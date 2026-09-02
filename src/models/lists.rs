use serde::{Deserialize, Serialize};

/// List metadata returned by the Lists chapter.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub favorite_count: u64,
    pub item_count: u64,
    pub iso_639_1: String,
    pub list_type: String,
}
