use serde::{Deserialize, Serialize};

/// A single change item returned by the TMDB Changes endpoints.
#[derive(Debug, Deserialize)]
pub struct ChangeItem {
    pub id: u64,
    pub action: String,
    pub time: String,
    pub iso_639_1: Option<String>,
    pub iso_3166_1: Option<String>,
    pub value: Option<String>,
}

/// A change group returned by the TMDB Changes endpoints.
#[derive(Debug, Deserialize)]
pub struct Change {
    pub key: String,
    pub items: Vec<ChangeItem>,
}

/// Query parameters to filter changes
#[derive(Debug, Serialize)]
pub struct ChangesQuery {
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub page: Option<u32>,
}
