use serde::{Deserialize, Serialize};

/// A TMDB company returned by the Companies chapter.
#[derive(Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct Company {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub headquarters: Option<String>,
    pub homepage: Option<String>,
    pub logo_path: Option<String>,
    pub origin_country: Option<String>,
    #[serde(rename = "parent_company")]
    pub parent: Option<Box<Self>>,
}
