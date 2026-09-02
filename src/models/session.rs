use serde::{Deserialize, Serialize};

/// Request payload used to create a session from a valid request token.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SessionRequest {
    pub request_token: String,
}

/// Response from a successful session creation request.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SessionResponse {
    pub success: bool,
    pub session_id: String,
}

/// Request payload for creating a guest session.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GuestSessionRequest {
    pub device_id: Option<String>,
}

/// Guest session response payload.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GuestSessionResponse {
    pub success: bool,
    pub guest_session_id: String,
    pub expires_at: String,
}
