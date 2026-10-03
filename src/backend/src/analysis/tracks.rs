use std::collections::HashMap;

use serde::Serialize;

use crate::{
    navidrome::interface::scrobble::ScrobbleWithSong
};

#[derive(Serialize, Clone)]
pub struct TrackStatArtists {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Clone)]
pub struct TrackStat {
    pub name: String,
    pub artists: Vec<TrackStatArtists>,
    pub album: String,
    pub album_id: String,
    pub id: String,
    pub plays: u64,
    pub played_hours: f64
}

impl TrackStat {
    pub fn group(
        scrobbles_with_songs: Vec<ScrobbleWithSong>
    ) -> HashMap<String, TrackStat> {
        let mut track_stat: HashMap<String, TrackStat> = HashMap::new();

        for scrobble in scrobbles_with_songs {
            let duration_hour = scrobble.track.duration / (60.0*60.0);

            match track_stat.get_mut(&scrobble.track.id.clone()) {
                Some(v) => {
                    (*v).plays += 1;
                    (*v).played_hours += duration_hour
                },
                None => {
                    track_stat.insert(
                        scrobble.track.id.clone(),
                        TrackStat {
                            name: scrobble.track.title.clone(),
                            artists: scrobble.track.artists.iter().map(|a|
                                TrackStatArtists {id: a.artist.id.clone(), name: a.artist.name.clone()}
                            ).collect(),
                            album: scrobble.track.album.clone(),
                            album_id: scrobble.track.album_id.clone(),
                            id: scrobble.track.id.clone(),
                            plays: 1,
                            played_hours: duration_hour
                        }
                    );
                }
            };
        }

        return track_stat;
    }
}
