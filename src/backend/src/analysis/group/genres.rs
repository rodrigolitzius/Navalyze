use std::collections::HashMap;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::{navidrome::interface::scrobble::ScrobbleWithSong, handlers::time::to_datetime_duration_vec};

pub fn group(
    scrobbles_with_songs: &Vec<ScrobbleWithSong>,
    timezone: Tz
) -> HashMap<String, Vec<(DateTime<Tz>, f64)>> {
    let mut genres: HashMap<String, (String, Vec<ScrobbleWithSong>)> = HashMap::new();

    for scrobble in scrobbles_with_songs {
        for genre in &scrobble.track.genres {
            let entry = genres.entry(genre.id.clone()).or_insert((genre.name.clone(), Vec::new()));

            entry.1.push(scrobble.clone());
        }
    }

    let result: HashMap<String, Vec<(DateTime<Tz>, f64)>>= genres.into_iter().map(|genre| {
        (genre.1.0, to_datetime_duration_vec(genre.1.1, timezone))
    }).collect();

    return result;
}
