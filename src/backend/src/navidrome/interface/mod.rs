// This module is to group the usage of both Navidrome's Subsonic and internal API
pub mod scrobble;
pub mod error;

use std::collections::HashMap;

use axum::{body::Bytes, http::HeaderValue};
use bitflags::bitflags;
use reqwest::{Method, RequestBuilder, header};
use uuid::Uuid;

use crate::{
    handlers::LoginRequest, navidrome::{
        interface::{error::NavidromeSessionError, scrobble::Scrobble},
        native::{NativeAlbum, NativeArtist, NativeSongArtist, NativeSongData, NavidromeNativeSession},
        subsonic::{NavidromeSubsonicSession, SubsonicAlbum, SubsonicArtist, SubsonicPlaylist}
    },
    reqwest::ReqwestAPiErrorExt
};

bitflags! {
    #[derive(Clone)]
    pub struct ArtistRole: u8 {
        const ARTIST   = 0b00000001;
        const ALBUM    = 0b00000010;
        const COMPOSER = 0b00000100;
    }
}

pub struct Artist {
    pub name: String,
    pub id: String,
    pub mbz_id: Option<Uuid>
}

pub struct Album {
    pub name: String,
    pub year: Option<u64>,
    pub artists: Vec<Artist>
}

pub struct Playlist {
    pub name: String,
    pub id: String,
    pub song_ids: Vec<String>
}

pub struct SongData {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_id: String,
    pub album_artist: String,
    pub duration: f64,
    pub artists: Vec<SongArtist>
}

pub struct SongArtist {
    pub artist: Artist,
    pub role: ArtistRole
}

pub struct Image {
    pub bytes: Bytes,
    pub content_type: HeaderValue
}

pub type TrackHashmap = HashMap<String, SongData>;

pub struct NavidromeInterface {
    native_session: NavidromeNativeSession,
    subsonic_session: NavidromeSubsonicSession
}

impl NavidromeInterface {
    pub async fn new(request: LoginRequest, allow_invalid_certs: bool) -> Result<Self, NavidromeSessionError> {
        return Ok(Self {
            native_session: NavidromeNativeSession::new(request.clone(), allow_invalid_certs).await?,
            subsonic_session: NavidromeSubsonicSession::new(request.clone(), allow_invalid_certs).await?
        });
    }

    pub async fn build_track_hashmap(&self, scrobbles: &Vec<Scrobble>) -> Result<TrackHashmap, NavidromeSessionError> {
        let songs = self.native_session.song(Vec::new()).await?;

        let media_file_ids: Vec<&String> = scrobbles.iter().map(|s| {&s.media_file_id}).collect();

        let songs: HashMap<String, NativeSongData> = songs.into_iter().filter(|s| media_file_ids.contains(&&s.0)).collect();

        let mut result: TrackHashmap = HashMap::new();
        for kv in songs {
            let _ = result.insert(kv.0, SongData::from_native(kv.1));
        }

        return Ok(result);
    }

    pub fn subsonic_relay(&self, method: reqwest::Method, endpoint: &String) -> RequestBuilder {
        let url = format!("{}/rest/{}", self.subsonic_session.url, endpoint);

        let request_builder = self.subsonic_session.client
            .request(method, url)
            .query(&self.subsonic_session.default_params);

        return request_builder
    }

    pub async fn get_artist(&self, id: &str) -> Result<Artist, NavidromeSessionError> {
        let artist = self.subsonic_session.get_artist(id).await?;

        return Ok(Artist::from_subsonic(artist));
    }

    pub async fn get_album(&self, id: &String) -> Result<Album, NavidromeSessionError> {
        let native_album = self.native_session.album(&id).await?;
        let subsonic_album = self.subsonic_session.get_album(&id).await?;

        return Ok(Album::from_navidrome(native_album, subsonic_album));
    }

    pub async fn scrobbles(&self, after_ts: u64) -> Result<Vec<Scrobble>, NavidromeSessionError> {
        return self.native_session.scrobble(after_ts).await;
    }

    pub async fn get_playlist(&self, id: &String) -> Result<Playlist, NavidromeSessionError> {
        let playlist = self.subsonic_session.get_playlist(id).await?;

        return Ok(Playlist::from_subsonic(playlist));
    }

    pub async fn playlists(&self) -> Result<Vec<Playlist>, NavidromeSessionError> {
        let subsonic_playlists = self.subsonic_session.get_playlist_ids().await?;

        let mut result: Vec<Playlist> = Vec::new();

        for subsonic_playlist_id in subsonic_playlists {
            let playlist = self.get_playlist(&subsonic_playlist_id).await?;

            result.push(playlist);
        }

        return Ok(result);
    }

    pub async fn get_art(&self, id: &str, size: Option<&String>) -> Result<Image, NavidromeSessionError> {
        let mut client_queries: Vec<(String, String)> = Vec::new();
        client_queries.push(("id".to_string(), id.to_string()));

        if let Some(n) = size {
            client_queries.push(("size".to_string(), n.clone()));
        }

        let response = self.subsonic_relay(Method::GET, &"getCoverArt".to_string())
            .query(&client_queries)
            .send()
            .await
            .map_reqwest_api_err()?;

        let content_type = match response.headers().get(header::CONTENT_TYPE) {
            Some(v) => v,
            None => {return Err(NavidromeSessionError::NoContentType)}
        }.to_owned();

        return Ok(Image {
            bytes: response.bytes().await.unwrap(),
            content_type: content_type
        })
    }
}

trait FromNative {
    type Native;

    fn from_native(native: Self::Native) -> Self;
}

trait FromSubsonic {
    type Subsonic;

    fn from_subsonic(subsonic: Self::Subsonic) -> Self;
}

trait FromNavidrome {
    type Subsonic;
    type Native;

    fn from_navidrome(native: Self::Native, subsonic: Self::Subsonic) -> Self;
}

impl FromNative for SongArtist {
    type Native = NativeSongArtist;

    fn from_native(native: Self::Native) -> Self {
        return Self {
            artist: Artist::from_native(native.artist),
            role: native.role
        };
    }
}

impl FromNative for Artist {
    type Native = NativeArtist;

    fn from_native(native: Self::Native) -> Self {
        return Self {
            name: native.name,
            id: native.id,
            mbz_id: native.mbz_id
        };
    }
}

impl FromSubsonic for Artist {
    type Subsonic = SubsonicArtist;

    fn from_subsonic(subsonic: Self::Subsonic) -> Self {
        return Self {
            name: subsonic.name,
            id: subsonic.id,
            mbz_id: subsonic.music_brainz_id
        };
    }
}

impl FromNavidrome for Album {
    type Native = NativeAlbum;
    type Subsonic = SubsonicAlbum;

    fn from_navidrome(native: Self::Native, subsonic: Self::Subsonic) -> Self {
        return Self {
            name: native.name,
            year: subsonic.year,
            artists: native.artists.into_iter().map(|a| Artist::from_native(a)).collect()
        }
    }
}

impl FromSubsonic for Playlist {
    type Subsonic = SubsonicPlaylist;

    fn from_subsonic(subsonic: Self::Subsonic) -> Self {
        return Self {
            name: subsonic.name,
            id: subsonic.id,
            song_ids: subsonic.entry.into_iter().map(|e| e.id).collect()
        }
    }
}

impl FromNative for SongData {
    type Native = NativeSongData;

    fn from_native(native: Self::Native) -> Self {
        return Self {
            id: native.id,
            title: native.title,
            artist: native.artist,
            album: native.album,
            album_id: native.album_id,
            album_artist: native.album_artist,
            duration: native.duration,
            artists: native.artists.into_iter().map(|a| SongArtist::from_native(a)).collect()
        };
    }
}

impl From<Vec<&str>> for ArtistRole {
    fn from(value: Vec<&str>) -> Self {
        let mut result = ArtistRole::empty();

        for s in value {
            if s == "artist" {result |= ArtistRole::ARTIST}
            if s == "album" {result |= ArtistRole::ALBUM}
            if s == "composer" {result |= ArtistRole::COMPOSER}
        }

        return result;
    }
}
