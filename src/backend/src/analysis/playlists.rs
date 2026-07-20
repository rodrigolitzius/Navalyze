use std::collections::HashMap;

use serde::Serialize;

use crate::{
    navidrome::interface::{scrobble::Scrobble, TrackHashmap, Playlist}
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
        scrobbles: Vec<&Scrobble>,
        playlists: &Vec<Playlist>,
        track_hashmap: &TrackHashmap
    ) -> HashMap<String, PlaylistStat> {
        let mut playlist_stat: HashMap<String, PlaylistStat> = HashMap::new();

        for scrobble in scrobbles {
            let song_data = match track_hashmap.get(&scrobble.media_file_id) {
                Some(v) => v,
                None => continue
            };

            let duration_hour = song_data.duration / (60.0*60.0);

            for playlist in playlists {
                if !playlist.song_ids.contains(&song_data.id) {continue;}

                match playlist_stat.get_mut(&playlist.id) {
                    Some(v) => {
                        (*v).plays += 1;
                        (*v).played_hours += duration_hour
                    },
                    None => {
                        playlist_stat.insert(playlist.id.clone(), PlaylistStat {
                            id: playlist.id.clone(),
                            name: playlist.name.clone(),
                            played_hours: duration_hour,
                            plays: 1
                        });
                    }
                };
            }
        }

        return playlist_stat;
    }
}
