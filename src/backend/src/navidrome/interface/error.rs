use reqwest::StatusCode;
use crate::{reqwest::ReqwestApiError, api::error::ApiError};

pub enum NavidromeSessionError {
    Reqwest(reqwest::Error),
    Unreachable(reqwest::Error),
    ParseJson(serde_json::Error),
    Status(StatusCode),
    Unauthorized,
}

impl From<ReqwestApiError> for NavidromeSessionError {
    fn from(value: ReqwestApiError) -> Self {
        return match value {
            ReqwestApiError::Connection(e) => NavidromeSessionError::Unreachable(e),
            ReqwestApiError::Other(e) => NavidromeSessionError::Unreachable(e),
            ReqwestApiError::Unauthorized(_) => NavidromeSessionError::Unauthorized,
            ReqwestApiError::Status(s) => NavidromeSessionError::Status(s),
            ReqwestApiError::ParseJson(e) => NavidromeSessionError::ParseJson(e)
        }
    }
}

impl From<serde_json::Error> for NavidromeSessionError {
    fn from(value: serde_json::Error) -> Self {
        return Self::ParseJson(value);
    }
}

impl From<NavidromeSessionError> for ApiError {
    fn from(value: NavidromeSessionError) -> Self {
        return match value {
            NavidromeSessionError::Reqwest(e) => ApiError::Internal(
                format!("Reqwest failed: {}", e.to_string())
            ),
            NavidromeSessionError::Unreachable(e) => ApiError::NavidromeUnreachable(
                format!("Navidrome could not be reached: {}", e.to_string())
            ),
            NavidromeSessionError::Unauthorized => ApiError::Unauthorized(
                "Invalid credentials".into()
            ),

            NavidromeSessionError::ParseJson(e) => ApiError::Internal(
                format!("Could not parse Navidrome's response: {}", e.to_string())
            ),

            NavidromeSessionError::Status(s) => ApiError::Internal(
                format!("Navidrome returned an unexpected status code: {}", s.as_str())
            )
        }
    }
}
