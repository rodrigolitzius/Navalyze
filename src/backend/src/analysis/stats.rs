use std::collections::HashSet;

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
        scrobbles_with_songs: &Vec<ScrobbleWithSong>
    ) -> Stats {
        let mut tracks = HashSet::new();
        let mut albums = HashSet::new();
        let mut artists = HashSet::new();

        let mut played_hours = 0.0;
        let mut plays = 0;

        for scrobble in scrobbles_with_songs {
            played_hours += scrobble.track.duration / (60.0*60.0);
            plays += 1;

            tracks.insert(scrobble.track.id.clone());
            albums.insert(scrobble.track.album_id.clone());

            for song_artist in &scrobble.track.artists {
                let _ = artists.insert(song_artist.artist.id.clone());
            }
        }

        return Stats {
            albums: albums.len(),
            tracks: tracks.len(),
            artists: artists.len(),
            plays, played_hours
        };
    }
}
