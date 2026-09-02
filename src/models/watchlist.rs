use serde::{Deserialize, Serialize};

/// Payload for adding media to a watchlist.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct WatchlistRequest {
    pub media_type: String,
    pub media_id: u64,
    pub watchlist: bool,
}

/// Common response for favorite/watchlist write operations.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct WatchlistResponse {
    pub success: bool,
    pub status_code: u16,
    pub status_message: String,
}
