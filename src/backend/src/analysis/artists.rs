use std::{collections::HashMap};

use serde::Serialize;

use crate::{
    navidrome::{Scrobble, SongData},
    analysis::GroupScrobble
};

#[derive(Serialize, Clone)]
pub struct ArtistStat {
    pub name: String,
    pub id: String,
    pub plays: u64,
    pub played_hours: f64
}

impl<'a> GroupScrobble<'a> for ArtistStat {
    type Result = HashMap<String, ArtistStat>;
    type Source = (Vec<&'a Scrobble>, &'a HashMap<String, SongData>);
    type Include = ();

    fn group(source: Self::Source, _include: Self::Include) -> Self::Result {
        let mut artist_stat: Self::Result = HashMap::new();

        for scrobble in source.0.iter() {
            let song_data = match source.1.get(&scrobble.media_file_id) {
                Some(v) => v,
                None => continue
            };

            let duration_hour = song_data.duration / (60.0*60.0);

            for artist in song_data.participants.artists.iter() {
                match artist_stat.get_mut(&artist.id) {
                    Some(v) => {
                        (*v).plays += 1;
                        (*v).played_hours += duration_hour
                    },
                    None => {
                        artist_stat.insert(
                            artist.id.clone(),
                            ArtistStat {
                                id: artist.id.clone(),
                                name: artist.name.clone(),
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
