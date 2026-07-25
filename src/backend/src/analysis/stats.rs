use std::collections::HashMap;

use serde::Serialize;

use crate::{
    navidrome::interface::{scrobble::Scrobble, TrackHashmap}
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
        scrobbles: Vec<&Scrobble>,
        track_hashmap: &TrackHashmap
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

        for scrobble in scrobbles {
            let song_data = match track_hashmap.get(&scrobble.media_file_id) {
                Some(v) => v,
                None => continue
            };

            result.plays += 1;
            result.played_hours += song_data.duration / (60.0*60.0);

            tracks.insert(song_data.id.clone(), "");
            albums.insert(song_data.album_id.clone(), "");

            for artist in &song_data.artists {
                let _ = artists.insert(artist.id.clone(), "");
            }
        }

        result.tracks = tracks.len();
        result.albums = albums.len();
        result.artists = artists.len();

        return result;
    }
}
