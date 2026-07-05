pub mod native;

use reqwest::{self};
use serde::{Deserialize, Serialize};

use crate::navidrome::ArtistRole;

pub struct NavidromeNativeSession {
    pub user_id: String,
    pub url: String,
    pub client: reqwest::Client,

    #[allow(unused)]
    pub token: String
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub id: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumData {
    pub id: String,
    pub name: String,

    #[serde(rename = "minOriginalYear")]
    pub year: u64
}

pub struct SongData {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_id: String,
    pub duration: f64,
    pub artists: Vec<Artist>
}

pub struct Artist {
    pub id: String,
    pub name: String,
    pub role: ArtistRole
}
