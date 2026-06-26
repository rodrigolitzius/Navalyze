pub mod native;
pub mod subsonic;

use rusqlite::Row;

use crate::api::{
    error::ApiError,
    Range
};

#[derive(Clone)]
#[allow(unused)]
pub struct Scrobble {
    pub media_file_id: String,
    pub user_id: String,
    pub submission_time: u64
}

pub enum NavidromeSessionError {
    Reqwest(reqwest::Error),
    Unreachable(reqwest::Error),
    ParseJson(serde_json::Error),
    Unauthorized,
}

impl Scrobble {
    pub fn filter_range(scrobbles: &Vec<Scrobble>, range: Range<u64>) -> Vec<&Scrobble> {
        let mut refs: Vec<&Scrobble> = Vec::new();

        for scrobble in scrobbles {
            if range.contains(&scrobble.submission_time) {
                refs.push(&scrobble);
            }
        }

        return refs;
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
            )
        }
    }
}

impl From<serde_json::Error> for NavidromeSessionError {
    fn from(value: serde_json::Error) -> Self {
        return Self::ParseJson(value);
    }
}

pub fn validate_reqwest_response(response: Result<reqwest::Response, reqwest::Error>) -> Result<reqwest::Response, NavidromeSessionError> {
    let response = match response {
        Ok(v) => v,
        Err(e) => {
            return Err(NavidromeSessionError::Unreachable(e));
        }
    };

    let response = match response.error_for_status() {
        Ok(v) => v,
        Err(_) => {
            return Err(NavidromeSessionError::Unauthorized);
        }
    };

    return Ok(response);
}

pub fn build_scrobble(row: &Row) -> Result<Scrobble, rusqlite::Error> {
    let media_file_id: String = row.get("media_file_id")?;
    let user_id: String = row.get("user_id")?;
    let submission_time: i64 = row.get("submission_time")?;

    return Ok(Scrobble {
        media_file_id, user_id,
        submission_time: submission_time as u64
    });
}
