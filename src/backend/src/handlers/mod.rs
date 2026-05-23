pub mod login;
pub mod recent;
pub mod relay;

use std::{str::FromStr, collections::HashMap};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use axum::{
    extract::{FromRef, FromRequestParts, Query, Json, State, Path},
    http::{StatusCode, HeaderMap, Method},
    body::Bytes
};
use reqwest::header;

use crate::{
    api::{ApiState, LoginSession, error::*},
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
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = ApiState::from_ref(state);

        let auth_header = match parts.headers.get("Authorization") {
            None => {return Err(StatusCode::UNAUTHORIZED)},
            Some(v) => v
        };

        let header_string = match auth_header.to_str() {
            Ok(v) => v,
            Err(_) => {return Err(StatusCode::UNAUTHORIZED)}
        };

        let uuid = match Uuid::from_str(header_string) {
            Ok(v) => v,
            Err(_) => {return Err(StatusCode::UNAUTHORIZED);}
        };

        return match state.sessions.read().await.contains_key(&uuid) {
            true => Ok(Auth{uuid: uuid}),
            false => Err(StatusCode::UNAUTHORIZED)
        }
    }
}

pub fn get_param_default<T>(hashmap: &HashMap<String, String>, key: &str, default: T) -> T
where T: FromStr {
    let limit = hashmap.get(key);
    if let None = limit { return default; }

    let limit: T = limit.unwrap().parse().unwrap_or(default);

    return limit;
}
