use std::str::FromStr;
use std::collections::HashMap;

use axum::extract::{FromRequestParts, Query};

use crate::api::error::ApiError;

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

impl Range {
    pub fn contains(&self, other: &u64) -> bool {
        return (self.start <= *other) && (self.end >= *other);
    }
}

fn get_param_default<T>(hashmap: &HashMap<String, String>, key: &str, default: T) -> T
where T: FromStr {
    let limit = hashmap.get(key);
    if let None = limit { return default; }

    let limit: T = limit.unwrap().parse().unwrap_or(default);

    return limit;
}
