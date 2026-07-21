use std::str::FromStr;
use std::collections::HashMap;

use axum::extract::{FromRef, FromRequestParts, Query};
use uuid::Uuid;

use crate::{
    api::{ApiState, Sessions, RwLockLoginSession, error::ApiError},
};

// ==== STRUCTS =====
pub struct SessionExtractor(pub RwLockLoginSession);

pub struct ResponseFilter {
    limit: usize,
    offset: usize
}

#[derive(Clone, Copy)]
pub struct Range {
    pub start: u64,
    pub end: u64
}

pub struct HandlerParams {
    pub range: Range,
    pub filter: ResponseFilter
}

// ==== IMPLS =====
impl<S> FromRequestParts<S> for SessionExtractor
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
            true => Ok(SessionExtractor(get_session_from_uuid(&uuid, &state.sessions).await?)),
            false => {return Err(ApiError::Unauthorized("You don't have permission to access this".into()))}
        }
    }
}

impl<S> FromRequestParts<S> for HandlerParams
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Query(queries) = match Query::<HashMap<String, String>>::from_request_parts(parts, state).await {
            Ok(v) => v,
            Err(_) => return Err(ApiError::BadRequest("Invalid queries".into()))
        };

        return Ok(Self {
            filter: ResponseFilter::from_query(&queries).await,
            range: Range::from_query(&queries).await
        })
    }
}

impl Range {
    async fn from_query(queries: &HashMap<String, String>) -> Self {
        let (start, end) = (
            get_param_default(&queries, "a", u64::MIN),
            get_param_default(&queries, "b", u64::MAX),
        );

        let range = Self {start, end};

        return range;
    }

    pub fn contains(&self, other: &u64) -> bool {
        return (self.start <= *other) && (self.end >= *other);
    }
}

impl ResponseFilter {
    async fn from_query(queries: &HashMap<String, String>) -> Self {
        let limit = get_param_default(&queries, "limit", usize::MAX);
        let offset = get_param_default(&queries, "offset", 0);

        return ResponseFilter {limit: limit, offset: offset};
    }

    pub fn select<'a, T>(&self, response: &'a Vec<T>) -> Vec<&'a T>{
        return response.iter().skip(self.offset).take(self.limit).collect();
    }
}

// ==== FUNCTIONS =====
fn get_param_default<T>(hashmap: &HashMap<String, String>, key: &str, default: T) -> T
where T: FromStr {
    let limit = hashmap.get(key);
    if let None = limit { return default; }

    let limit: T = limit.unwrap().parse().unwrap_or(default);

    return limit;
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
