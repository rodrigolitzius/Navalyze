use std::collections::HashMap;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::{handlers::time::to_datetime_duration_vec, navidrome::interface::scrobble::ScrobbleWithSong};

pub fn group(
    scrobbles_with_songs: &Vec<ScrobbleWithSong>,
    timezone: Tz
) -> HashMap<String, Vec<(DateTime<Tz>, f64)>> {
    let mut languages: HashMap<String, Vec<ScrobbleWithSong>> = HashMap::new();

    for scrobble in scrobbles_with_songs {
        let release_type = scrobble.track.languages_to_str().unwrap_or("Unknown".into());

        let entry = languages.entry(release_type).or_insert(Vec::new());

        entry.push(scrobble.clone());
    }

    let result: HashMap<String, Vec<(DateTime<Tz>, f64)>>= languages.into_iter().map(|language| {
        (language.0, to_datetime_duration_vec(language.1, timezone))
    }).collect();

    return result;
}
