use std::collections::HashMap;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::{navidrome::interface::scrobble::ScrobbleWithSong, handlers::time::to_datetime_duration_vec};

pub fn group(
    scrobbles_with_songs: &Vec<ScrobbleWithSong>,
    timezone: Tz
) -> HashMap<String, Vec<(DateTime<Tz>, f64)>> {
    let mut release_types: HashMap<String, Vec<ScrobbleWithSong>> = HashMap::new();

    for scrobble in scrobbles_with_songs {
        for release_type in &scrobble.track.release_types {
            let entry = release_types.entry(release_type.release_type.clone()).or_insert(Vec::new());

            entry.push(scrobble.clone());
        }
    }

    let result: HashMap<String, Vec<(DateTime<Tz>, f64)>>= release_types.into_iter().map(|release_type| {
        (release_type.0, to_datetime_duration_vec(release_type.1, timezone))
    }).collect();

    return result;
}
