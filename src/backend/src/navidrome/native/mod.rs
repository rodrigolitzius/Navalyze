pub mod native;

use reqwest::{self};
use serde::{Deserialize, Serialize};

#[allow(unused)]
pub struct NavidromeNativeSession {
    pub user_id: String,
    pub url: String,
    pub client: reqwest::Client,
    pub token: String
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub id: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SongData {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub album_id: String,
    pub duration: f64,
    pub participants: Participants
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participants {
    #[serde(rename = "artist")]
    pub artists: Vec<Artist>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub missing: bool,
}
