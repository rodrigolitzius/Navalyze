use std::str::FromStr;

use rand::{RngExt, distr::Alphanumeric};

use reqwest::{Client, Method};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    navidrome::{
        subsonic::{SubsonicArtist, NavidromeSubsonicSession, SubsonicResponse, SubsonicResponseAlbumField, SubsonicAlbum, SubsonicResponsePlaylistsField, SubsonicPlaylists, SubsonicResponsePlaylistField},
        interface::{
            error::NavidromeSessionError
        }
    },
    handlers::LoginRequest,
    reqwest::{ReqwestAPiErrorExt, ResponseJsonExt}
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeserializeSubsonicArtist {
    id: String,
    name: String,
    music_brainz_id: Option<String>
}

#[derive(Deserialize)]
struct DeserializeSubsonicResponseArtistField {
    artist: DeserializeSubsonicArtist
}

impl From<DeserializeSubsonicArtist> for SubsonicArtist {
    fn from(value: DeserializeSubsonicArtist) -> Self {
        let mbz_id = match value.music_brainz_id {
            Some(s) => Uuid::from_str(&s.as_str()).ok(),
            None => None
        };

        return Self {
            id: value.id,
            name: value.name,
            music_brainz_id: mbz_id
        };
    }
}

impl NavidromeSubsonicSession {
    // TODO: Actually test if this fails if the login request has invalid credentials
    pub async fn new(login_request: LoginRequest) -> Result<Self, NavidromeSessionError> {
        let salt: String = rand::rng()
            .sample_iter(Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        let hash = format!("{:x}", md5::compute(format!("{}{}", login_request.password, salt)));

        let mut default_params: Vec<(String, String)> = Vec::new();
        default_params.push(("u".to_string(), login_request.username));
        default_params.push(("s".to_string(), salt.clone()));
        default_params.push(("t".to_string(), hash.clone()));
        default_params.push(("c".to_string(), crate::APP_NAME.to_string()));
        default_params.push(("v".to_string(), "1.8.0".to_string()));
        default_params.push(("f".to_string(), "json".to_string()));

        let url = format!("{}/rest/ping", login_request.url);

        let client = match Client::builder().tls_danger_accept_invalid_certs(true).build() {
            Ok(v) => v,
            Err(e) => {
                return Err(NavidromeSessionError::Reqwest(e));
            }
        };

        let _response = client
            .request(Method::GET, url)
            .query(&default_params)
            .send()
            .await
            .map_reqwest_api_err()?;

        let result = Self {
            default_params: default_params,
            url: login_request.url,
            client: client
        };

        return Ok(result)
    }

    pub async fn get_artist(&self, id: &String) -> Result<SubsonicArtist, NavidromeSessionError> {
        let url = format!("{}/rest/getArtist?id={}", self.url, id);

        let response = self.client
            .get(url)
            .query(&self.default_params)
            .send()
            .await
            .map_reqwest_api_err()?;

        let artist: SubsonicResponse<DeserializeSubsonicResponseArtistField> = response.into_json().await?;

        return Ok(artist.subsonic_response.artist.into());
    }

    pub async fn get_album(&self, id: &String) -> Result<SubsonicAlbum, NavidromeSessionError> {
        let url = format!("{}/rest/getAlbum?id={}", self.url, id);

        let response = self.client
            .get(url)
            .query(&self.default_params)
            .send()
            .await
            .map_reqwest_api_err()?;

        let artist: SubsonicResponse<SubsonicResponseAlbumField> = response.into_json().await?;

        return Ok(artist.subsonic_response.album);
    }

    pub async fn get_playlists(&self) -> Result<Vec<SubsonicPlaylists>, NavidromeSessionError> {
        let url = format!("{}/rest/getPlaylists", self.url);

        let response = self.client
            .get(url)
            .query(&self.default_params)
            .send()
            .await
            .map_reqwest_api_err()?;

        let playlists: SubsonicResponse<SubsonicResponsePlaylistsField> = response.into_json().await?;

        return Ok(playlists.subsonic_response.playlists.playlist);
    }

    pub async fn get_playlist_song_ids(&self, id: &String) -> Result<Vec<String>, NavidromeSessionError> {
        let url = format!("{}/rest/getPlaylist?id={}", self.url, id);

        let response = self.client
            .get(url)
            .query(&self.default_params)
            .send()
            .await
            .map_reqwest_api_err()?;

        let response: SubsonicResponse<SubsonicResponsePlaylistField> = response.into_json().await?;
        let song_ids: Vec<String> = response.subsonic_response.playlist.entry.into_iter().map(|e| e.id).collect();

        return Ok(song_ids);
    }
}
