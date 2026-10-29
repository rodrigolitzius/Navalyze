use std::collections::HashMap;

use crate::{
    navidrome::interface::scrobble::ScrobbleWithSong
};

pub fn get_release_types(scrobbles_with_songs: &Vec<ScrobbleWithSong>) -> HashMap<String, f64> {
    let mut result: HashMap<String, f64> = HashMap::new();

    for scrobble in scrobbles_with_songs {
        let duration = scrobble.track.duration / (60.0*60.0);

        let release_types = &scrobble.track.release_types;

        for release_type in release_types {
            *result.entry(release_type.release_type.clone()).or_insert(0.0) += duration;
        }
    }

    return result;
}
