pub mod subsonic;

use serde::{Serialize, Deserialize};

use uuid::Uuid;

pub struct NavidromeSubsonicSession {
    pub default_params: Vec<(String, String)>,
    pub url: String,
    pub client: reqwest::Client
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseArtist {
    pub id: String,
    pub name: String,
    pub music_brainz_id: Option<Uuid>
}
