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

use std::{str::FromStr, collections::HashMap};
use tokio::sync::RwLock;
use serde::{Deserialize};
use serde_json::json;
use uuid::Uuid;

use axum::{
    extract::{FromRef, FromRequestParts, Query, Json, State, Path},
    http::{StatusCode, HeaderMap, Method},
    body::Bytes
};
use reqwest::header;

use crate::{
    api::{ApiState, LoginSession, Sessions, RwLockLoginSession, error::*},
};

pub struct Auth{uuid: Uuid}

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

impl<S> FromRequestParts<S> for Auth
where
    ApiState: axum::extract::FromRef<S>,
    S: Send + Sync
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = ApiState::from_ref(state);

        let auth_header = match parts.headers.get("Authorization") {
            None => {return Err(ApiError::Unauthorized("Missing Authorization header".into()))},
            Some(v) => v
        };

        let header_string = match auth_header.to_str() {
            Ok(v) => v,
            Err(_) => {return Err(ApiError::Unauthorized("Invalid Authorization header".into()))}
        };

        let uuid = match Uuid::from_str(header_string) {
            Ok(v) => v,
            Err(_) => {return Err(ApiError::Unauthorized("Authorization header is not and UUID".into()));}
        };

        return match state.sessions.read().await.contains_key(&uuid) {
            true => Ok(Auth{uuid: uuid}),
            false => {return Err(ApiError::Unauthorized("You don't have permission to access this".into()))}
        }
    }
}

async fn get_session_from_uuid(uuid: &Uuid, sessions: &Sessions) -> Result<RwLockLoginSession, ApiError> {
    let session = sessions.read().await;

    let result = match session.get(uuid) {
        Some(v) => v,
        None => {
            return Err(ApiError::Internal("Could not find token".into()));
        }
    };

    return Ok(result.clone());
}
