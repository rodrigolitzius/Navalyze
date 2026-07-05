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
pub struct SubsonicResponseArtist {
    pub id: String,
    pub name: String,
    pub music_brainz_id: Option<Uuid>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsonicResponseArtistField {
    pub artist: SubsonicResponseArtist
}
