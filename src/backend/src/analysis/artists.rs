use std::{collections::HashMap};

use serde::Serialize;

use crate::{
    navidrome::interface::{scrobble::ScrobbleWithSong, ArtistRole}
};

#[derive(Serialize, Clone)]
pub struct ArtistStat {
    pub name: String,
    pub id: String,
    pub plays: u64,
    pub played_hours: f64
}

impl ArtistStat {
    pub fn group(
        scrobbles_with_songs: Vec<ScrobbleWithSong>,
        artist_types: &ArtistRole
    ) -> HashMap<String, ArtistStat> {
        let mut artist_stat: HashMap<String, ArtistStat> = HashMap::new();

        for scrobble in scrobbles_with_songs {
            let duration_hour = scrobble.track.duration / (60.0*60.0);

            for song_artist in &scrobble.track.artists {
                if !song_artist.role.intersects(artist_types.clone()) {continue;}

                match artist_stat.get_mut(&song_artist.artist.id) {
                    Some(v) => {
                        (*v).plays += 1;
                        (*v).played_hours += duration_hour
                    },
                    None => {
                        artist_stat.insert(
                            song_artist.artist.id.clone(),
                            ArtistStat {
                                id: song_artist.artist.id.clone(),
                                name: song_artist.artist.name.clone(),
                                plays: 1,
                                played_hours: duration_hour
                            }
                        );
                    }
                };
            }
        }

        return artist_stat;
    }
}
