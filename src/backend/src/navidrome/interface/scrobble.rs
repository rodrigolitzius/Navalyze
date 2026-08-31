use serde::Deserialize;
use chrono::{DateTime, TimeZone};

use crate::{
    handlers::extract::Range,
    navidrome::interface::SongData
};

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scrobble {
    pub media_file_id: String,
    pub submission_time: u64
}

pub struct ScrobbleWithSong<'a> {
    pub scrobble: &'a Scrobble,
    pub track: &'a SongData
}

pub trait ScrobbleWithSongFilter<'a> {
    fn filter_range(self, range: Range) -> Self;
    fn filter_album(self, album_ids: &Vec<&str>) -> Self;
    fn filter_artist(self, artist_ids: &Vec<&str>) -> Self;
    fn filter_track(self, track_ids: &Vec<&str>) -> Self;
}

impl<'a> ScrobbleWithSongFilter<'a> for Vec<ScrobbleWithSong<'a>> {
    fn filter_range(self, range: Range) -> Self {
        return self.into_iter().filter(|s| range.contains(&s.scrobble.submission_time)).collect();
    }

    fn filter_album(self, album_ids: &Vec<&str>) -> Self {
        return self.into_iter().filter(|s| album_ids.contains(&&s.track.album_id.as_str())).collect();
    }

    fn filter_artist(self, artist_ids: &Vec<&str>) -> Self {
        return self.into_iter().filter(|s| {
            let track_artist_ids: Vec<&str> = s.track.artists.iter().map(|a| a.artist.id.as_str()).collect();

            for artist_id in artist_ids {
                if track_artist_ids.contains(artist_id) {
                    return true;
                }
            }

            return false;
        }).collect();
    }

    fn filter_track(self, track_ids: &Vec<&str>) -> Self {
        return self.into_iter().filter(|s| track_ids.contains(&&s.track.id.as_str())).collect();
    }
}

impl Scrobble {
    pub fn date_time<T>(&self, tz: T) -> Option<DateTime<T>>
    where T: TimeZone {
        return Some(DateTime::from_timestamp_secs(self.submission_time as i64)?.with_timezone(&tz));
    }
}
