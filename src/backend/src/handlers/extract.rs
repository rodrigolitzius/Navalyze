use std::str::FromStr;
use std::collections::HashMap;

use axum::extract::{FromRef, FromRequestParts, OptionalFromRequestParts, Query};
use axum::http::request::Parts;
use chrono::{WeekdaySet, Weekday};
use chrono_tz::Tz;
use uuid::Uuid;

use crate::{
    api::{ApiState, Sessions, RwLockLoginSession, error::ApiError},
    navidrome::interface::ArtistRole
};

trait QueryParam: Sized {
    fn from_query(queries: &HashMap<String, String>) -> Result<Self, ApiError>;
}

trait QueryParamDefault: QueryParam + Default {
    fn from_query_default(queries: &HashMap<String, String>) -> Result<Self, ApiError> {
        return match Self::from_query(&queries) {
            Ok(v) => Ok(v),
            Err(e) => {
                match e {
                    ApiError::MissingParameter(_) => Ok(Self::default()),
                    _ => Err(e)
                }
            }
        };
    }
}

trait QueryParamOption: QueryParam {
    fn from_query_option(queries: &HashMap<String, String>) -> Result<Option<Self>, ApiError> {
        return match Self::from_query(&queries) {
            Ok(v) => Ok(Some(v)),
            Err(e) => {
                match e {
                    ApiError::MissingParameter(_) => Ok(None),
                    _ => Err(e)
                }
            }
        };
    }
}

impl<T: QueryParam + Default> QueryParamDefault for T {}
impl<T: QueryParam> QueryParamOption for T {}

// ==== STRUCTS =====
pub struct SessionExtractor(pub RwLockLoginSession);

pub struct ResponseFilter {
    pub limit: usize,
    pub offset: usize
}

#[derive(Clone, Copy)]
pub struct Range {
    pub start: u64,
    pub end: u64
}

pub struct StartTs(pub u64);
pub struct EndTS(pub u64);
pub struct Limit(pub usize);
pub struct Offset(pub usize);
pub struct Timezone(pub Tz);
pub struct Resolution(pub u64);
pub struct Weekdays(pub WeekdaySet);
pub struct ArtistRoles(pub ArtistRole);

pub struct StartTsWithDefault(pub u64);
pub struct EndTSWithDefault(pub u64);
pub struct LimitWithDefault(pub usize);
pub struct OffsetWithDefault(pub usize);
pub struct RangeWithDefault(pub Range);
pub struct ResponseFilterWithDefault(pub ResponseFilter);
pub struct ArtistRolesWithDefault(pub ArtistRole);

pub struct WeekdaysAndTimezone {pub set: WeekdaySet, pub timezone: Tz}

// ============================
// ===== FromRequestParts =====
// ============================
impl<S: Send + Sync> FromRequestParts<S> for SessionExtractor
where ApiState: axum::extract::FromRef<S> {
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

// These need to be explicitely passed with the url
impl<S: Send + Sync> FromRequestParts<S> for StartTs {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let queries = queries_from_request_parts(parts, state).await?;

        return Self::from_query(&queries)
    }
}

impl<S: Send + Sync> FromRequestParts<S> for EndTS {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let queries = queries_from_request_parts(parts, state).await?;

        return Self::from_query(&queries)
    }
}

impl<S: Send + Sync> FromRequestParts<S> for Offset {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let queries = queries_from_request_parts(parts, state).await?;

        return Self::from_query(&queries)
    }
}

impl<S: Send + Sync> FromRequestParts<S> for Limit {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let queries = queries_from_request_parts(parts, state).await?;

        return Self::from_query(&queries)
    }
}

impl<S: Send + Sync> FromRequestParts<S> for Timezone {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let queries = queries_from_request_parts(parts, state).await?;

        return Self::from_query(&queries)
    }
}

impl<S: Send + Sync> FromRequestParts<S> for Resolution {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let queries = queries_from_request_parts(parts, state).await?;

        return Self::from_query(&queries)
    }
}

impl<S: Send + Sync> FromRequestParts<S> for ArtistRoles {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let queries = queries_from_request_parts(parts, state).await?;

        return Self::from_query(&queries)
    }
}

// These can be ommited in the URL as they have a default value
impl<S: Send + Sync> FromRequestParts<S> for StartTsWithDefault {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let queries = queries_from_request_parts(parts, state).await?;

        return Ok(Self(StartTs::from_query_default(&queries)?.0));
    }
}

impl<S: Send + Sync> FromRequestParts<S> for EndTSWithDefault {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let queries = queries_from_request_parts(parts, state).await?;

        return Ok(Self(EndTS::from_query_default(&queries)?.0));
    }
}

impl<S: Send + Sync> FromRequestParts<S> for LimitWithDefault {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let queries = queries_from_request_parts(parts, state).await?;

        return Ok(Self(Limit::from_query_default(&queries)?.0));
    }
}

impl<S: Send + Sync> FromRequestParts<S> for OffsetWithDefault {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let queries = queries_from_request_parts(parts, state).await?;

        return Ok(Self(Offset::from_query_default(&queries)?.0));
    }
}

impl<S: Send + Sync> FromRequestParts<S> for ArtistRolesWithDefault {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let queries = queries_from_request_parts(parts, state).await?;

        return Ok(Self(ArtistRoles::from_query_default(&queries)?.0));
    }
}

impl<S: Send + Sync> FromRequestParts<S> for RangeWithDefault {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        return Ok(Self(Range {
            start: StartTsWithDefault::from_request_parts(parts, state).await?.0,
            end: EndTSWithDefault::from_request_parts(parts, state).await?.0
        }));
    }
}

impl<S: Send + Sync> FromRequestParts<S> for ResponseFilterWithDefault {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        return Ok(Self(ResponseFilter {
            offset: OffsetWithDefault::from_request_parts(parts, state).await?.0,
            limit: LimitWithDefault::from_request_parts(parts, state).await?.0
        }));
    }
}

// These don't have a default and can be optional
impl<S: Send + Sync> OptionalFromRequestParts<S> for Timezone {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let queries = queries_from_request_parts(parts, state).await?;

        return Timezone::from_query_option(&queries);
    }
}

// WeekdaysAndTimezone will return Some if both weekdays and timezones are passed
// If weekdays is passed but not timezone, it will return an error
impl<S: Send + Sync> OptionalFromRequestParts<S> for WeekdaysAndTimezone {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let queries = queries_from_request_parts(parts, state).await?;

        let weekdays = Weekdays::from_query(&queries);
        let timezone = Timezone::from_query(&queries);

        let weekdays = match weekdays {
            Ok(v) => v,
            Err(e) => {match e {
                ApiError::MissingParameter(_) => return Ok(None),
                _ => return Err(e)
            }}
        };

        let timezone = match timezone {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        return Ok(Some(Self {set: weekdays.0, timezone: timezone.0}));
    }
}

// ============================
// ======= Query Params =======
// ============================
impl QueryParam for StartTs {
    fn from_query(queries: &HashMap<String, String>) -> Result<Self, ApiError> {
        let start = queries.get("a")
            .ok_or_else(|| ApiError::MissingParameter("Missing \"a\" argument (Starting timestamp)".into()))?
            .parse().map_err(|_| ApiError::BadRequest("Failed to parse \"a\" argument".into()))?;

        return Ok(Self(start));
    }
}

impl QueryParam for EndTS {
    fn from_query(queries: &HashMap<String, String>) -> Result<Self, ApiError> {
        let end = queries.get("b")
            .ok_or_else(|| ApiError::MissingParameter("Missing \"b\" argument (Ending timestamp)".into()))?
            .parse().map_err(|_| ApiError::BadRequest("Failed to parse \"b\" argument".into()))?;

        return Ok(Self(end));
    }
}

impl QueryParam for Timezone {
    fn from_query(queries: &HashMap<String, String>) -> Result<Self, ApiError> {
        let timezone = queries.get("tz").ok_or_else(|| {
            ApiError::MissingParameter("No timezone specified".into())
        })?;

        let timezone = timezone.parse().map_err(|_| {
            ApiError::BadRequest("Failed to parse timezone".into())
        })?;

        return Ok(Self(timezone));
    }
}

impl QueryParam for Weekdays {
    fn from_query(queries: &HashMap<String, String>) -> Result<Self, ApiError> {
        let weekdays_vec = queries.get("weekdays").ok_or_else(|| ApiError::MissingParameter("Missing weekdays".into()))?
            .split(",")
            .collect::<Vec<&str>>()
            .into_iter().map(|w| { w.parse::<u8>() })
            .collect::<Result<Vec<u8>, <u8 as FromStr>::Err>>().map_err(|_| {ApiError::BadRequest("Invalid string".into())})?
            .into_iter().map(|w| { Weekday::try_from(w) })
            .collect::<Result<Vec<Weekday>, <Weekday as TryFrom<u8>>::Error>>().map_err(|e| {ApiError::BadRequest(format!("Could not convert value to a weekday: {}", e.to_string()).into())})?;

        let mut weekdays = WeekdaySet::EMPTY;

        for weekday in weekdays_vec {
            weekdays.insert(weekday);
        }

        return Ok(Self(weekdays))
    }
}

impl QueryParam for Resolution {
    fn from_query(queries: &HashMap<String, String>) -> Result<Self, ApiError> {
        let resolution = queries.get("res").ok_or_else(|| {
            ApiError::BadRequest("No resolution specified".into())
        })?;

        let resolution: u64 = resolution.parse().map_err(|_| {
            ApiError::BadRequest("Invalid resolution string".into())
        })?;

        return Ok(Self(resolution));
    }
}

impl QueryParam for Offset {
    fn from_query(queries: &HashMap<String, String>) -> Result<Self, ApiError> {
        return Ok(Self(
            queries.get("offset")
                .ok_or_else(|| ApiError::MissingParameter("Missing offset parameter".into()))?
                .parse().map_err(|_| ApiError::BadRequest("Failed to parse offset paramater".into()))?
        ))
    }
}

impl QueryParam for Limit {
    fn from_query(queries: &HashMap<String, String>) -> Result<Self, ApiError> {
        return Ok(Self(
            queries.get("limit")
                .ok_or_else(|| ApiError::MissingParameter("Missing limit parameter".into()))?
                .parse().map_err(|_| ApiError::BadRequest("Failed to parse limit paramater".into()))?
        ))
    }
}

impl QueryParam for ArtistRoles {
    fn from_query(queries: &HashMap<String, String>) -> Result<Self, ApiError> {
        // let artist_types_default = String::from("artist");
        let artist_strings: Vec<&str> = queries.get("type")
            .ok_or_else(|| ApiError::MissingParameter("Artist type(s) not specified".into()))?
            .split(",").collect();

        return Ok(Self(ArtistRole::from(artist_strings)))
    }
}

// ============================
// ====== Param Defaults ======
// ============================
impl Default for Weekdays {fn default() -> Self {return Self(WeekdaySet::ALL)}}
impl Default for StartTs {fn default() -> Self {return Self(u64::MIN)}}
impl Default for EndTS {fn default() -> Self {return Self(u64::MAX)}}
impl Default for Limit {fn default() -> Self {return Self(usize::MAX)}}
impl Default for Offset {fn default() -> Self {return Self(usize::MIN)}}
impl Default for ArtistRoles {fn default() -> Self {return Self(ArtistRole::ARTIST)}}

// ============================
// ========== Other ===========
// ============================
impl Range {
    pub fn contains(&self, other: &u64) -> bool {
        return (self.start <= *other) && (self.end >= *other);
    }
}

impl ResponseFilter {
    pub fn select<'a, T>(&self, response: Vec<T>) -> Vec<T>{
        return response.into_iter().skip(self.offset).take(self.limit).collect();
    }
}

// ==== FUNCTIONS =====
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

pub async fn queries_from_request_parts<S: Send + Sync>(parts: &mut Parts, state: &S) -> Result<HashMap<String, String>, ApiError> {
    let Query(queries) = Query::<HashMap<String, String>>::from_request_parts(parts, state)
        .await.map_err(|e| ApiError::BadRequest(format!("Invalid queries: {}", e)))?;

    return Ok(queries)
}
