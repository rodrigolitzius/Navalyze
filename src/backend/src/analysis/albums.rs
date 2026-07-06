use std::{collections::HashMap};

use serde::Serialize;

use crate::{
    navidrome::interface::{scrobble::Scrobble, TrackHashmap}
};

#[derive(Serialize, Clone)]
pub struct AlbumStat {
    pub name: String,
    pub artist: String,
    pub id: String,
    pub artist_id: String,
    pub plays: u64,
    pub played_hours: f64
}

impl AlbumStat {
    pub fn group(
        scrobbles: Vec<&Scrobble>,
        track_hashmap: &TrackHashmap
    ) -> HashMap<String, AlbumStat> {
        let mut album_stat: HashMap<String, AlbumStat> = HashMap::new();

        for scrobble in scrobbles.iter() {
            let song_data = match track_hashmap.get(&scrobble.media_file_id) {
                Some(v) => v,
                None => continue
            };

            let duration_hour = song_data.duration / (60.0*60.0);

            match album_stat.get_mut(&song_data.album_id.clone()) {
                Some(v) => {
                    (*v).plays += 1;
                    (*v).played_hours += duration_hour
                },
                None => {
                    album_stat.insert(
                        song_data.album_id.clone(),
                        AlbumStat {
                            name: song_data.album.clone(),
                            artist: song_data.album_artist.clone(),
                            artist_id: song_data.album_artist_id.clone(),
                            id: song_data.album_id.clone(),
                            plays: 1,
                            played_hours: duration_hour
                        }
                    );
                }
            };
        }

        return album_stat;
    }
}
