use std::collections::HashMap;

use serde::Serialize;

use crate::{
    navidrome::interface::scrobble::ScrobbleWithSong
};

#[derive(Serialize, Clone)]
pub struct GenreStat {
    pub name: String,
    pub id: String,
    pub plays: u64,
    pub played_hours: f64
}

impl GenreStat {
    pub fn group(
        scrobbles_with_songs: &Vec<ScrobbleWithSong>
    ) -> HashMap<String, GenreStat> {
        let mut genre_stat: HashMap<String, GenreStat> = HashMap::new();

        for scrobble in scrobbles_with_songs {
            let duration_hour = scrobble.track.duration / (60.0*60.0);

            for genre in &scrobble.track.genres {
                let entry = genre_stat.entry(genre.id.clone()).or_insert(GenreStat {
                    name: genre.name.clone(),
                    id: genre.id.clone(),
                    plays: 0,
                    played_hours: 0.0
                });

                entry.played_hours += duration_hour;
                entry.plays += 1;
            }
        }

        return genre_stat;
    }
}
