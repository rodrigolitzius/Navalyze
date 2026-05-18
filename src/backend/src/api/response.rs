use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::{self, json};

pub enum ApiError {
    // Login
    LoginNavidromeUnreachable(String),

    // Other
    Internal(String),
    Unauthorized(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (code, data) = match self {
            ApiError::LoginNavidromeUnreachable(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg)
        };

        return (
            code,
            Json(json!({"error": data}))
        ).into_response()
    }
}
