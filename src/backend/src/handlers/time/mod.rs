pub mod frequency;
pub mod artist;
pub mod album;
pub mod track;
pub mod playlist;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::navidrome::interface::{TrackHashmap, scrobble::Scrobble};


pub fn to_datetime_duration_vec(scrobbles: &Vec<&Scrobble>, tracks_hashmap: &TrackHashmap, timezone: Tz) -> Vec<(DateTime<Tz>, f64)> {
    let mut result = Vec::new();

    for scrobble in scrobbles {
        let date_time = scrobble.date_time(timezone).unwrap();
        let duration = match tracks_hashmap.get(&scrobble.media_file_id) {
            Some(v) => v,
            None => continue
        }.duration / (60.0*60.0);

        result.push((date_time, duration));
    }

    return result;
}
