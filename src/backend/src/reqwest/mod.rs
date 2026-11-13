use std::{error::Error as StdError};

use reqwest::{Error, StatusCode, Response};
use serde::{de::DeserializeOwned};

#[derive(Debug)]
pub enum ReqwestApiError {
    Connection(Error),
    Unauthorized(StatusCode),
    ParseJson(serde_json::Error),
    Status(StatusCode),
    Other(Error),
}

type ReqwestResponseError = Result<Response, Error>;

pub trait ReqwestAPiErrorExt {
    fn map_reqwest_api_err(self) -> Result<Response, ReqwestApiError>;
    fn from_status_code(status: StatusCode) -> Option<ReqwestApiError>;
    fn handle_err(error: Error) -> ReqwestApiError;
}

pub trait ReqwestErrorExt {
    fn error_msg(self) -> String;
}

impl ReqwestAPiErrorExt for ReqwestResponseError {
    fn map_reqwest_api_err(self) -> Result<Response, ReqwestApiError> {
        let error = match self {
            Ok(v) => {
                if let Some(e) = Self::from_status_code(v.status()) {
                    Err(e)
                } else {
                    Ok(v)
                }
            },

            Err(e) => {
                Err(Self::handle_err(e))
            }
        };

        return error;
    }

    fn from_status_code(status: StatusCode) -> Option<ReqwestApiError> {
        return match status {
            StatusCode::UNAUTHORIZED => Some(ReqwestApiError::Unauthorized(status)),
            s => {
                if s.is_success() {return None;}

                return Some(ReqwestApiError::Status(s));
            }
        };
    }

    fn handle_err(error: Error) -> ReqwestApiError {
        if error.is_connect() {
            return ReqwestApiError::Connection(error)
        } else {
            return ReqwestApiError::Other(error)
        }
    }
}

impl ReqwestErrorExt for Error {
    fn error_msg(self) -> String {
        let mut full_error = String::from(self.to_string());

        let mut error = self.source();
        while let Some(source) = error {
            full_error.push_str(format!(" => {}", source.to_string()).as_str());
            error = source.source();
        }

        return full_error;
    }
}

pub trait ResponseJsonExt {
    async fn into_json<T>(self) -> Result<T, ReqwestApiError>
    where T: DeserializeOwned;
}

impl ResponseJsonExt for Response {
    async fn into_json<T>(self) -> Result<T, ReqwestApiError>
    where T: DeserializeOwned {
        let string = self.text().await;

        let string = match string {
            Ok(v) => v,
            Err(e) => return Err(ReqwestResponseError::handle_err(e))
        };

        let json = match serde_json::from_str::<T>(&string) {
            Ok(v) => v,
            Err(e) => return Err(ReqwestApiError::ParseJson(e))
        };

        return Ok(json);
    }
}
