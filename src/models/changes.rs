use serde::{Deserialize, Serialize};

/// A single change item returned by the TMDB Changes endpoints.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChangeItem {
    pub id: u64,
    pub action: String,
    pub time: String,
    pub iso_639_1: Option<String>,
    pub value: Option<String>,
    pub original_value: Option<String>,
}

/// A changes response wrapper.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChangesResponse<T> {
    pub page: u32,
    pub results: Vec<T>,
    pub total_pages: u32,
    pub total_results: u32,
}
