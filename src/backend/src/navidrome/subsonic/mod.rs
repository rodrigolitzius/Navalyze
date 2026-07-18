pub mod subsonic;

use serde::{Serialize, Deserialize};

use uuid::Uuid;

pub struct NavidromeSubsonicSession {
    pub default_params: Vec<(String, String)>,
    pub url: String,
    pub client: reqwest::Client
}

#[derive(Deserialize)]
pub struct SubsonicResponse<T> {
    #[serde(rename = "subsonic-response")]
    subsonic_response: T
}

// ==== ARTIST ====
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubsonicArtist {
    pub id: String,
    pub name: String,
    pub music_brainz_id: Option<Uuid>
}

// ==== ALBUM ====
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubsonicAlbum {
    pub id: String,
    pub name: String,
    pub year: Option<u64>,
    pub artist: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsonicResponseAlbumField {
    pub album: SubsonicAlbum
}
