pub mod subsonic;

use serde::{Serialize, Deserialize};

use uuid::Uuid;

#[allow(unused)]
pub struct NavidromeSubsonicSession {
    pub default_params: Vec<(String, String)>,
    pub url: String,
    pub client: reqwest::Client,
    pub salt: String,
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseArtist {
    pub name: String,
    pub album_count: u64,
    pub album: Vec<ResponseArtistAlbum>,
    pub music_brainz_id: Option<Uuid>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseArtistAlbum {
    pub id: String,
    pub year: u64
}
