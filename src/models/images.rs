use serde::{Deserialize, Serialize};

/// Base image metadata returned by image endpoints.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ImageResponse {
    pub id: Option<u64>,
    pub backdrops: Vec<crate::models::common::ImageInfo>,
    pub posters: Vec<crate::models::common::ImageInfo>,
}
