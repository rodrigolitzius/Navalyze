mod session;

use axum::http::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use reqwest::{Client, ClientBuilder, StatusCode};

use crate::reqwest::ReqwestApiError;

const MBZ_URL: &'static str = "https://api.listenbrainz.org/1";

pub struct MbzSession {
    client: Client,

    #[allow(unused)]
    token: Uuid
}

pub enum MbzError {
    Reqwest(reqwest::Error),
    ParseJson(serde_json::Error),
    UnexpectedStatus(StatusCode),
    EmptyArtistList
}

impl From<reqwest::Error> for MbzError {
    fn from(value: reqwest::Error) -> Self {
        return MbzError::Reqwest(value);
    }
}

impl From<serde_json::Error> for MbzError {
    fn from(value: serde_json::Error) -> Self {
        return MbzError::ParseJson(value);
    }
}

impl From<ReqwestApiError> for MbzError {
    fn from(value: ReqwestApiError) -> Self {
        return match value {
            ReqwestApiError::Connection(e) => MbzError::Reqwest(e),
            ReqwestApiError::Other(e) => MbzError::Reqwest(e),
            ReqwestApiError::Unauthorized(s) => MbzError::UnexpectedStatus(s),
            ReqwestApiError::Status(s) => MbzError::UnexpectedStatus(s),
            ReqwestApiError::ParseJson(e) => MbzError::ParseJson(e)
        };
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MbzArtist {
    pub gender: Option<String>,
    #[serde(rename = "type")]
    pub artist_type: Option<String>
}
