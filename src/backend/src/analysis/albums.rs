use std::{collections::HashMap};

use serde::Serialize;

use crate::{
    navidrome::{Scrobble, native::SongData},
    analysis::GroupScrobble
};

#[derive(Serialize, Clone)]
pub struct AlbumStat {
    pub name: String,
    pub artist: String,
    pub id: String,
    pub plays: u64,
    pub played_hours: f64
}

impl<'a> GroupScrobble<'a> for AlbumStat {
    type Result = HashMap<String, AlbumStat>;
    type Source = (Vec<&'a Scrobble>, &'a HashMap<String, SongData>);
    type Include = Option<Vec<String>>;

    fn group(source: Self::Source, include: Self::Include) -> Self::Result {
        let mut album_stat: Self::Result = HashMap::new();

        for scrobble in source.0.iter() {
            let song_data = match source.1.get(&scrobble.media_file_id) {
                Some(v) => v,
                None => continue
            };

            match &include {
                Some(v) => {
                    if !v.contains(&song_data.album_id.clone()) {continue}
                }
                None => {}
            }

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
                            artist: song_data.artist.clone(),
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
