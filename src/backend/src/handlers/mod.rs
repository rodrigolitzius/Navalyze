pub mod login;
pub mod recent;
pub mod relay;
pub mod artists;
pub mod artist;
pub mod albums;
pub mod album;
pub mod tracks;
pub mod playlists;
pub mod playlist;
pub mod extract;

use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Deserialize};
use serde_json::json;
use uuid::Uuid;

use axum::{
    extract::{Query, Json, State, Path},
    http::{StatusCode, HeaderMap, Method},
    body::Bytes
};
use reqwest::header;

use crate::{
    api::{ApiState, LoginSession, error::*},
};

#[derive(Deserialize)]
pub struct RawLoginRequest {
    pub username: String,
    pub password: String,
    pub url: String
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub url: String
}

impl From<RawLoginRequest> for LoginRequest {
    fn from(value: RawLoginRequest) -> Self {
        return LoginRequest {
            username: value.username,
            password: value.password,
            url: value.url.trim_end_matches('/').to_string()
        };
    }
}
