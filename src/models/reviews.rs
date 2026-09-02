use serde::{Deserialize, Serialize};

/// Review details from the TMDB Reviews chapter.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Review {
    pub id: String,
    pub author: String,
    pub author_details: AuthorDetails,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub url: String,
}

/// Nested author metadata attached to a review.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AuthorDetails {
    pub name: String,
    pub username: String,
    pub avatar_path: Option<String>,
    pub rating: Option<u8>,
}
