pub mod native;
pub mod subsonic;
pub mod scrobble;

use reqwest::StatusCode;
use rusqlite::Row;
use uuid::Uuid;
use serde::Deserialize;

use crate::{
    api::{error::ApiError},
    navidrome::{native::AlbumData, subsonic::SubsonicResponseArtist, scrobble::Scrobble},
    reqwest::ReqwestApiError,
};

#[derive(Deserialize)]
pub struct SubsonicResponse<T> {
    #[serde(rename = "subsonic-response")]
    subsonic_response: T
}

#[derive(Clone)]
pub struct Artist {
    #[allow(unused)]
    pub id: String,
    pub name: String,
    pub music_brainz_id: Option<Uuid>,
    pub albums: Vec<Album>
}

#[derive(Clone)]
pub struct Album {
    pub id: String,
    #[allow(unused)]
    pub name: String,
    pub year: u64,
}

pub enum NavidromeSessionError {
    Reqwest(reqwest::Error),
    Unreachable(reqwest::Error),
    ParseJson(serde_json::Error),
    Status(StatusCode),
    Unauthorized,
}

impl Artist {
    pub fn from_navidrome(artist: SubsonicResponseArtist, albums: Vec<AlbumData>) -> Artist {
        let mut new_albums: Vec<Album> = Vec::new();
        for album in albums {
            new_albums.push(album.into());
        }

        let result = Artist {
            albums: new_albums,
            id: artist.id,
            music_brainz_id: artist.music_brainz_id,
            name: artist.name
        };

        return result;
    }
}

impl From<AlbumData> for Album {
    fn from(value: AlbumData) -> Self {
        return Self {
            id: value.id,
            name: value.name,
            year: value.year
        };
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
            ),

            NavidromeSessionError::Status(s) => ApiError::Internal(
                format!("Navidrome returned an unexpected status code: {}", s.as_str())
            )
        }
    }
}

impl From<ReqwestApiError> for NavidromeSessionError {
    fn from(value: ReqwestApiError) -> Self {
        return match value {
            ReqwestApiError::Connection(e) => NavidromeSessionError::Unreachable(e),
            ReqwestApiError::Other(e) => NavidromeSessionError::Unreachable(e),
            ReqwestApiError::Unauthorized(_) => NavidromeSessionError::Unauthorized,
            ReqwestApiError::Status(s) => NavidromeSessionError::Status(s),
            ReqwestApiError::ParseJson(e) => NavidromeSessionError::ParseJson(e)
        }
    }
}

impl From<serde_json::Error> for NavidromeSessionError {
    fn from(value: serde_json::Error) -> Self {
        return Self::ParseJson(value);
    }
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
