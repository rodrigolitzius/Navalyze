use std::collections::HashMap;

use serde::Serialize;

use crate::{
    navidrome::interface::scrobble::ScrobbleWithSong
};

#[derive(Serialize)]
pub struct Stats {
    pub plays: u64,
    pub played_hours: f64,
    pub tracks: usize,
    pub albums: usize,
    pub artists: usize
}

impl Stats {
    pub fn group(
        scrobbles_with_songs: Vec<ScrobbleWithSong>
    ) -> Stats {
        let mut result = Stats {
            plays: 0,
            played_hours: 0.0,
            tracks: 0,
            albums: 0,
            artists: 0
        };

        let mut tracks = HashMap::new();
        let mut albums = HashMap::new();
        let mut artists = HashMap::new();

        for scrobble in scrobbles_with_songs {
            result.plays += 1;
            result.played_hours += scrobble.track.duration / (60.0*60.0);

            tracks.insert(scrobble.track.id.clone(), "");
            albums.insert(scrobble.track.album_id.clone(), "");

            for song_artist in &scrobble.track.artists {
                let _ = artists.insert(song_artist.artist.id.clone(), "");
            }
        }

        result.tracks = tracks.len();
        result.albums = albums.len();
        result.artists = artists.len();

        return result;
    }
}
