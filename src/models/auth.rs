use serde::{Deserialize, Serialize};

/// The payload returned by the authentication validation endpoint.
///
/// See the TMDB Authentication chapter for the API contract used to validate an
/// API key or session.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ValidateKeyResponse {
    pub success: bool,
    pub status_code: u16,
    pub status_message: String,
}

/// Generic authentication request status for write operations.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AuthStatus {
    pub success: bool,
    pub status_code: u16,
    pub status_message: String,
}
