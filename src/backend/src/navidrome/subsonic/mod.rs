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

// ==== PLAYLISTS ====
// This is for deserializing /rest/getPlaylists
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubsonicPlaylists {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsonicResponsePlaylistsVec {
    pub playlist: Vec<SubsonicPlaylists>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsonicResponsePlaylistsField {
    pub playlists: SubsonicResponsePlaylistsVec
}

// ==== PLAYLIST ====
// // This is for deserializing /rest/getPlaylist
#[derive(Debug, Serialize, Deserialize)]
pub struct SubsonicResponsePlaylistField {
    pub playlist: SubsonicPlaylist
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubsonicPlaylist {
    pub id: String,
    pub name: String,
    pub song_count: u64,
    #[serde(default)] // This for playlists with no songs
    pub entry: Vec<SubsonicPlaylistEntry>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsonicPlaylistEntry {
    // #[serde(rename = "id")]
    pub id: String
}
