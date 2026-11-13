use reqwest::{Error, header::ToStrError};
use serde_json::Error as SerdeJsonError;

use crate::{api::error::ApiError, reqwest::{ReqwestApiError, ReqwestErrorExt}};

pub enum MakeAWrappedError {
    Reqwest(ReqwestApiError),
    ImageContentTypeToStr(ToStrError),
    PayloadToText(SerdeJsonError),
    SetMime(Error),
    ResponseToBytes(Error)
}

impl From<MakeAWrappedError> for ApiError {
    fn from(value: MakeAWrappedError) -> Self {
        match value {
            MakeAWrappedError::ImageContentTypeToStr(e) => ApiError::Internal(
                format!("Failed to make Navidrome's artwork content type into a string: {}", e.to_string())
            ),

            MakeAWrappedError::SetMime(e) => ApiError::Internal(
                format!("Failed to set mime type for multipart form: {}", e.to_string())
            ),

            MakeAWrappedError::PayloadToText(e) => ApiError::Internal(
                format!("Failed to convert request payload into string {}", e.to_string())
            ),

            MakeAWrappedError::ResponseToBytes(e) => ApiError::Internal(
                format!("Failed to convert Make A Wrapped response to bytes: {}", e.to_string())
            ),

            MakeAWrappedError::Reqwest(e) => match e {
                ReqwestApiError::Connection(e2) => ApiError::Internal(
                    format!("Make a Wrapped API: Failed to connect to: {}", e2.error_msg())
                ),

                ReqwestApiError::ParseJson(e2) => ApiError::Internal(
                    format!("Make a Wrapped API: Could not parse json response: {}", e2.to_string())
                ),

                ReqwestApiError::Unauthorized(e2) => ApiError::Unauthorized(
                    format!("Make a Wrapped API: Unauthorized status {}", e2)
                ),

                ReqwestApiError::Status(e2) => ApiError::Internal(
                    format!("Make a Wrapped API: Unexpected status code: {}", e2)
                ),

                ReqwestApiError::Other(e2) => ApiError::Internal(
                    format!("Make a Wrapped API: Other reqwest error: {}", e2.error_msg())
                )
            }
        }
    }
}
