use std::collections::HashMap;

use serde::Serialize;

use crate::{
    navidrome::interface::{scrobble::ScrobbleWithSong, Playlist}
};

#[derive(Serialize, Clone)]
pub struct PlaylistStat {
    pub name: String,
    pub id: String,
    pub plays: u64,
    pub played_hours: f64
}

impl PlaylistStat {
    pub fn group(
        scrobbles_with_songs: &Vec<ScrobbleWithSong>,
        playlists: &Vec<Playlist>
    ) -> HashMap<String, PlaylistStat> {
        let mut playlist_stat: HashMap<String, PlaylistStat> = HashMap::new();

        for scrobble in scrobbles_with_songs {
            let duration_hour = scrobble.track.duration / (60.0*60.0);

            for playlist in playlists {
                if !playlist.song_ids.contains(&scrobble.track.id) {continue;}

                let entry = playlist_stat.entry(playlist.id.clone()).or_insert(PlaylistStat {
                    id: playlist.id.clone(),
                    name: playlist.name.clone(),
                    played_hours: 0.0,
                    plays: 0
                });

                entry.played_hours += duration_hour;
                entry.plays += 1;
            }
        }

        return playlist_stat;
    }
}
