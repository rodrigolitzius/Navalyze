use std::collections::HashMap;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::{analysis::time::date, navidrome::interface::scrobble::ScrobbleWithSong};

pub fn group(
    scrobbles_with_songs: &Vec<ScrobbleWithSong>,
    resolution: u64,
    timezone: Tz
) -> HashMap<String, HashMap<u64, f64>> {
    let mut genres: HashMap<String, (String, Vec<(DateTime<Tz>, f64)>)> = HashMap::new();

    for scrobble in scrobbles_with_songs {
        let duration = scrobble.track.duration / (60.0*60.0);

        for genre in &scrobble.track.genres {
            let entry = genres.entry(genre.id.clone()).or_insert((genre.name.clone(), Vec::new()));

            entry.1.push((scrobble.scrobble.date_time(timezone).unwrap(), duration));
        }
    }

    let mut result: HashMap<String, HashMap<u64, f64>> = HashMap::new();

    for (_, (genre_name, datetimes)) in genres.into_iter() {
        let grouped = date::group(&datetimes, resolution);

        result.entry(genre_name).or_insert(grouped);
    }

    return result;
}
