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
                match genre_stat.get_mut(&genre.id) {
                    Some(v) => {
                        (*v).plays += 1;
                        (*v).played_hours += duration_hour
                    },
                    None => {
                        genre_stat.insert(
                            genre.id.clone(),
                            GenreStat {
                                name: genre.name.clone(),
                                id: genre.id.clone(),
                                plays: 1,
                                played_hours: duration_hour
                            }
                        );
                    }
                };
            }
        }

        return genre_stat;
    }
}
