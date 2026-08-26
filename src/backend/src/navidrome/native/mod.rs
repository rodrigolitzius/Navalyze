pub mod native;

use reqwest::{self};
use serde::{Deserialize};
use uuid::Uuid;

use crate::navidrome::interface::ArtistRole;

pub struct NavidromeNativeSession {
    pub url: String,
    pub client: reqwest::Client,

    #[allow(unused)]
    pub token: String
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String
}

pub struct NativeAlbum {
    pub name: String,
    pub artists: Vec<NativeArtist>
}

pub struct NativeSongArtist {
    pub artist: NativeArtist,
    pub role: ArtistRole
}

pub struct NativeArtist {
    pub mbz_id: Option<Uuid>,
    pub id: String,
    pub name: String,
}

pub struct NativeSongData {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub artist_id: String,
    pub album: String,
    pub album_id: String,
    pub album_artist: String,
    pub album_artist_id: String,
    pub duration: f64,
    pub artists: Vec<NativeSongArtist>
}
