use std::collections::HashMap;

use crate::{
    navidrome::interface::scrobble::ScrobbleWithSong
};

pub fn get_languages(scrobbles_with_songs: &Vec<ScrobbleWithSong>) -> HashMap<String, f64> {
    let mut result: HashMap<String, f64> = HashMap::new();

    for scrobble in scrobbles_with_songs {
        let duration = scrobble.track.duration / (60.0*60.0);

        let track_languages_str = match scrobble.track.languages_to_str() {
            Some(v) => v,
            None => continue
        };

        *result.entry(track_languages_str).or_insert(0.0) += duration;
    }

    return result;
}
