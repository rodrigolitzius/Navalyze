pub mod native;

use reqwest::{self};
use serde::{Deserialize};

use crate::navidrome::interface::ArtistRole;

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

pub struct NativeSongArtist {
    pub id: String,
    pub name: String,
    pub role: ArtistRole
}

pub struct NativeSongData {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_id: String,
    pub duration: f64,
    pub artists: Vec<NativeSongArtist>
}
