use std::{collections::HashMap};

use serde::Serialize;

use crate::{
    navidrome::interface::scrobble::ScrobbleWithSong
};

#[derive(Serialize, Clone)]
pub struct AlbumStat {
    pub name: String,
    pub artist: String,
    pub id: String,
    pub plays: u64,
    pub played_hours: f64
}

impl AlbumStat {
    pub fn group(
        scrobbles_with_songs: &Vec<ScrobbleWithSong>
    ) -> HashMap<String, AlbumStat> {
        let mut album_stat: HashMap<String, AlbumStat> = HashMap::new();

        for scrobble in scrobbles_with_songs {
            let duration_hour = scrobble.track.duration / (60.0*60.0);

            let entry = album_stat.entry(scrobble.track.album_id.clone()).or_insert(AlbumStat {
                name: scrobble.track.album.clone(),
                artist: scrobble.track.album_artist.clone(),
                id: scrobble.track.album_id.clone(),
                plays: 0,
                played_hours: 0.0
            });

            entry.played_hours += duration_hour;
            entry.plays += 1;
        }

        return album_stat;
    }
}
