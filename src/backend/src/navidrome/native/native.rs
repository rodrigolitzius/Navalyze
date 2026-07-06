use reqwest::Client;

use std::{collections::HashMap};

use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::{
    handlers::LoginRequest, navidrome::{
        interface::{
            ArtistRole, error::NavidromeSessionError
        }, native::{LoginResponse, NativeSongArtist, NativeSongData, NavidromeNativeSession},
    }, reqwest::{ReqwestAPiErrorExt, ResponseJsonExt}
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeserializeSongData {
    id: String,
    title: String,
    artist: String,
    artist_id: String,
    album: String,
    album_id: String,
    album_artist: String,
    album_artist_id: String,
    duration: f64,
    participants: DeserializeParticipants
}

#[derive(Serialize, Deserialize)]
struct DeserializeParticipants {
    #[serde(rename = "artist")]
    artists: Option<Vec<DeserializeArtist>>,

    #[serde(rename = "albumartist")]
    album_artists: Option<Vec<DeserializeArtist>>,

    #[serde(rename = "composer")]
    composers: Option<Vec<DeserializeArtist>>
}

#[derive(Serialize, Deserialize)]
struct DeserializeArtist {
    id: String,
    name: String
}

impl NativeSongArtist {
    fn from_deserialize_artist(artist: DeserializeArtist, role: ArtistRole) -> Self {
        return Self {id: artist.id, name: artist.name, role: role};
    }
}

impl NavidromeNativeSession {
    pub async fn new(login_request: LoginRequest) -> Result<Self, NavidromeSessionError> {
        let client = match Client::builder().tls_danger_accept_invalid_certs(true).build() {
            Ok(v) => v,
            Err(e) => {
                return Err(NavidromeSessionError::Reqwest(e));
            }
        };

        let mut body = HashMap::new();
        body.insert("username", login_request.username);
        body.insert("password", login_request.password);

        let response = client
            .post(format!("{}/auth/login", login_request.url))
            .json(&body)
            .send()
            .await
            .map_reqwest_api_err()?;

        let login_response: LoginResponse = response.into_json().await?;

        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            "x-nd-authorization",
            HeaderValue::from_str(format!("Bearer {}", login_response.token).as_str()).expect("Navidrome returned an invalid token")
        );

        let client = Client::builder()
            .tls_danger_accept_invalid_certs(true)
            .default_headers(default_headers)
            .build()
            .unwrap();

        return Ok(Self {
            url: login_request.url.to_string(),
            user_id: login_response.id,
            client: client,
            token: login_response.token,
        });
    }

    pub async fn song(&self, queries: Vec<(String, String)>) -> Result<HashMap<String, NativeSongData>, NavidromeSessionError> {
        let url = format!("{}/api/song/", self.url);

        let response = self.client
            .get(&url)
            .query(&queries)
            .send()
            .await
            .map_reqwest_api_err()?;

        let response = response.into_json::<Vec<DeserializeSongData>>().await?;

        let mut result = HashMap::new();

        for song_data in response {
            let mut all_artists: Vec<NativeSongArtist> = Vec::new();

            match song_data.participants.artists {
                Some(v) => {
                    let mut inter_artists = v.into_iter().map(|a| NativeSongArtist::from_deserialize_artist(a, ArtistRole::ARTIST)).collect();
                    all_artists.append(&mut inter_artists);
                },
                None => {}
            }

            match song_data.participants.album_artists {
                Some(v) => {
                    let mut inter_artists = v.into_iter().map(|a| NativeSongArtist::from_deserialize_artist(a, ArtistRole::ALBUM)).collect();
                    all_artists.append(&mut inter_artists);
                },
                None => {}
            }

            match song_data.participants.composers {
                Some(v) => {
                    let mut inter_artists = v.into_iter().map(|a| NativeSongArtist::from_deserialize_artist(a, ArtistRole::COMPOSER)).collect();
                    all_artists.append(&mut inter_artists);
                },
                None => {}
            }

            let mut artists: HashMap<String, NativeSongArtist> = HashMap::new();

            for artist in all_artists {
                match artists.get_mut(&artist.id) {
                    None => {
                        artists.insert(artist.id.clone(), NativeSongArtist { id: artist.id, name: artist.name, role: artist.role });
                    }

                    Some(v) => {
                        (*v).role |= artist.role
                    }
                }
            }

            let song_data = NativeSongData {
                id: song_data.id,
                title: song_data.title,
                artist: song_data.artist,
                artist_id: song_data.artist_id,
                album: song_data.album,
                album_id: song_data.album_id,
                album_artist: song_data.album_artist,
                album_artist_id: song_data.album_artist_id,
                duration: song_data.duration,
                artists: artists.into_values().collect()
            };

            result.insert(song_data.id.clone(), song_data);
        }

        return Ok(result);
    }
}
