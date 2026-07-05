use crate::{api::Range, navidrome::interface::TrackHashmap};

#[derive(Clone)]
pub struct Scrobble {
    pub media_file_id: String,
    pub user_id: String,
    pub submission_time: u64
}

impl Scrobble {
    pub fn as_ref_vec<'a>(scrobbles: &'a Vec<Scrobble>) -> Vec<&'a Scrobble> {
        return scrobbles.iter().map(|s| s).collect();
    }

    pub fn filter_range(scrobbles: Vec<&Scrobble>, range: Range<u64>) -> Vec<&Scrobble> {
        let mut refs: Vec<&Scrobble> = Vec::new();

        for scrobble in scrobbles {
            if range.contains(&scrobble.submission_time) {
                refs.push(&scrobble);
            }
        }

        return refs;
    }

    pub fn filter_album<'a>(scrobbles: Vec<&'a Scrobble>, tracks_hashmap: &TrackHashmap, album_ids: &Vec<&String>) -> Vec<&'a Scrobble> {
        return scrobbles.into_iter().filter(|s| {
            let song_data = match tracks_hashmap.get(&s.media_file_id) {
                Some(v) => v,
                None => return false
            };

            return album_ids.contains(&&song_data.album_id);
        }).collect();
    }

    #[allow(unused)]
    pub fn filter_artist<'a>(scrobbles: Vec<&'a Scrobble>, tracks_hashmap: &TrackHashmap, artist_ids: &Vec<&String>) -> Vec<&'a Scrobble> {
        return scrobbles.into_iter().filter(|s| {
            let song_data = match tracks_hashmap.get(&s.media_file_id) {
                Some(v) => v,
                None => return false
            };

            for artist in &song_data.artists {
                if artist_ids.contains(&&artist.id) {return true};
            }

            return false;
        }).collect();
    }

    #[allow(unused)]
    pub fn filter_track<'a>(scrobbles: Vec<&'a Scrobble>, tracks_hashmap: &TrackHashmap, track_ids: &Vec<&String>) -> Vec<&'a Scrobble> {
        return scrobbles.into_iter().filter(|s| {
            let song_data = match tracks_hashmap.get(&s.media_file_id) {
                Some(v) => v,
                None => return false
            };

            return track_ids.contains(&&song_data.id);
        }).collect();
    }
}
