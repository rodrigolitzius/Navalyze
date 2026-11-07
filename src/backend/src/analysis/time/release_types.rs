use std::collections::HashMap;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::{analysis::time::date, navidrome::interface::scrobble::ScrobbleWithSong};

pub fn group(
    scrobbles_with_songs: &Vec<ScrobbleWithSong>,
    resolution: u64,
    timezone: Tz
) -> HashMap<String, HashMap<u64, f64>> {
    let mut release_types: HashMap<String, Vec<(DateTime<Tz>, f64)>> = HashMap::new();

    for scrobble in scrobbles_with_songs {
        let duration = scrobble.track.duration / (60.0*60.0);

        for release_type in &scrobble.track.release_types {
            let entry = release_types.entry(release_type.release_type.clone()).or_insert(Vec::new());

            entry.push((scrobble.scrobble.date_time(timezone).unwrap(), duration));
        }
    }

    let mut result: HashMap<String, HashMap<u64, f64>> = HashMap::new();

    for (release_type, datetimes) in release_types.into_iter() {
        let grouped = date::group(&datetimes, resolution);

        result.entry(release_type).or_insert(grouped);
    }

    return result;
}
