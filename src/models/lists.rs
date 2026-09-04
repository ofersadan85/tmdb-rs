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

/// Payload for adding media to a list.
#[derive(Debug, Serialize)]
pub struct AddToListRequest<'a> {
    pub media_type: &'a str,
    pub media_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchlist: Option<bool>,
}

impl<'a> AddToListRequest<'a> {
    /// Creates a new `AddToListRequest` for adding a media item to a list.
    pub const fn new(media_type: &'a str, media_id: u64) -> Self {
        Self {
            media_type,
            media_id,
            favorite: None,
            watchlist: None,
        }
    }
}
