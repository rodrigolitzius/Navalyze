// This module is to group the usage of both Navidrome's Subsonic and internal API
pub mod scrobble;
pub mod error;

use std::collections::HashMap;

use bitflags::bitflags;
use reqwest::RequestBuilder;
use uuid::Uuid;

use crate::{
    handlers::LoginRequest, navidrome::{
        interface::{
            error::NavidromeSessionError, scrobble::Scrobble
        }, native::{NativeSongArtist, NativeSongData, NavidromeNativeSession}, subsonic::NavidromeSubsonicSession,
    }
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
    pub mbz_id: Option<Uuid>
}

pub struct Album {
    pub name: String,
    pub artist: String,
    pub year: u64
}

pub struct SongData {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_id: String,
    pub duration: f64,
    pub artists: Vec<SongArtist>
}

pub struct SongArtist {
    pub id: String,
    pub name: String,
    pub role: ArtistRole
}

pub type TrackHashmap = HashMap<String, SongData>;

pub struct NavidromeInterface {
    native_session: NavidromeNativeSession,
    subsonic_session: NavidromeSubsonicSession
}

impl NavidromeInterface {
    pub async fn new(request: LoginRequest) -> Result<Self, NavidromeSessionError> {
        return Ok(Self {
            native_session: NavidromeNativeSession::new(request.clone()).await?,
            subsonic_session: NavidromeSubsonicSession::new(request.clone()).await?
        });
    }

    pub async fn build_track_hashmap(&self, scrobbles: &Vec<Scrobble>) -> Result<TrackHashmap, NavidromeSessionError> {
        let songs = self.native_session.song(Vec::new()).await?;

        let media_file_ids: Vec<&String> = scrobbles.iter().map(|s| {&s.media_file_id}).collect();

        let songs: HashMap<String, NativeSongData> = songs.into_iter().filter(|s| media_file_ids.contains(&&s.0)).collect();

        let mut result: TrackHashmap = HashMap::new();
        for kv in songs {
            let _ = result.insert(kv.0, SongData::from(kv.1));
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

    pub async fn get_artist(&self, id: &String) -> Result<Artist, NavidromeSessionError> {
        let artist = self.subsonic_session.get_artist(id).await?;

        return Ok(Artist {
            name: artist.name,
            mbz_id: artist.music_brainz_id
        });
    }

    pub async fn get_album(&self, id: &String) -> Result<Album, NavidromeSessionError> {
        let subsonic_album = self.subsonic_session.get_album(&id).await?;

        return Ok(Album {
            name: subsonic_album.name,
            artist: subsonic_album.artist,
            year: subsonic_album.year
        });
    }

    pub fn user_id(&self) -> &String {
        return &self.native_session.user_id;
    }
}

impl From<NativeSongArtist> for SongArtist {
    fn from(value: NativeSongArtist) -> Self {
        return Self { id: value.id, name: value.name, role: value.role };
    }
}

impl From<NativeSongData> for SongData {
    fn from(value: NativeSongData) -> Self {
        return Self {
            id: value.id,
            title: value.title,
            artist: value.artist,
            album: value.album,
            album_id: value.album_id,
            duration: value.duration,
            artists: value.artists.into_iter().map(|a| SongArtist::from(a)).collect()
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
