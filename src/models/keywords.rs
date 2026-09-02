use serde::{Deserialize, Serialize};

/// Keyword payload returned by the keywords endpoints.
#[derive(Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct Keyword {
    pub id: u64,
    pub name: String,
}

/// A keyword collection response.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct KeywordsResult {
    pub id: u64,
    pub keywords: Vec<Keyword>,
}
