use serde::{Deserialize, Serialize};

/// Payload for adding media to favorites.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FavoriteRequest {
    pub media_type: String,
    pub media_id: u64,
    pub favorite: bool,
}

/// Favorite action response payload.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FavoriteResponse {
    pub success: bool,
    pub status_code: u16,
    pub status_message: String,
}
