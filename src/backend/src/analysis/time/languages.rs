use std::collections::HashMap;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::{analysis::time::date, navidrome::interface::scrobble::ScrobbleWithSong};

pub fn group(
    scrobbles_with_songs: &Vec<ScrobbleWithSong>,
    resolution: u64,
    timezone: Tz
) -> HashMap<String, HashMap<u64, f64>> {
    let mut languages: HashMap<String, Vec<(DateTime<Tz>, f64)>> = HashMap::new();

    for scrobble in scrobbles_with_songs {
        let duration = scrobble.track.duration / (60.0*60.0);

        let release_type = scrobble.track.languages_to_str().unwrap_or("Unknown".into());

        let entry = languages.entry(release_type).or_insert(Vec::new());

        entry.push((scrobble.scrobble.date_time(timezone).unwrap(), duration));
    }

    let mut result: HashMap<String, HashMap<u64, f64>> = HashMap::new();

    for (release_type, datetimes) in languages.into_iter() {
        let grouped = date::group(&datetimes, resolution);

        result.entry(release_type).or_insert(grouped);
    }

    return result;
}
