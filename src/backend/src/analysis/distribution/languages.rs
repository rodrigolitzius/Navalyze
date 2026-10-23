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

        match result.get_mut(&track_languages_str) {
            Some(v) => *v += duration,
            None => {result.insert(track_languages_str.clone(), duration);}
        }
    }

    return result;
}
