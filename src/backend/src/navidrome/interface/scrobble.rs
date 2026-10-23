use std::collections::HashMap;

use serde::Deserialize;
use chrono::{DateTime, Datelike, TimeZone, WeekdaySet};

use crate::{
    handlers::extract::Range,
    navidrome::interface::{SongData, SongLanguage}
};

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scrobble {
    pub media_file_id: String,
    pub submission_time: u64
}

#[derive(Clone)]
pub struct ScrobbleWithSong<'a> {
    pub scrobble: &'a Scrobble,
    pub track: &'a SongData
}

pub trait ScrobbleWithSongFilter<'a> {
    fn filter_range(self, range: Range) -> Self;
    fn filter_weekdays<T: TimeZone>(self, weekdays: WeekdaySet, timezone: T) -> Self;
    fn filter_language(self, languages: &Vec<&SongLanguage>) -> Self;
    fn filter_genre(self, genres: &Vec<&str>) -> Self;
    fn filter_album(self, album_ids: &Vec<&str>) -> Self;
    fn filter_artist(self, artist_ids: &Vec<&str>) -> Self;
    fn filter_track(self, track_ids: &Vec<&str>) -> Self;
}

pub trait ScrobbleWithSongExt<'a> {
    fn get_all_languages(&self) -> HashMap<String, Vec<SongLanguage>>;
}

impl<'a> ScrobbleWithSongFilter<'a> for Vec<ScrobbleWithSong<'a>> {
    fn filter_range(self, range: Range) -> Self {
        return self.into_iter().filter(|s| range.contains(&s.scrobble.submission_time)).collect();
    }

    fn filter_weekdays<T: TimeZone>(self, weekdays: WeekdaySet, timezone: T) -> Self {
        self.into_iter().filter(|s| {
            weekdays.contains(s.scrobble.date_time(timezone.clone()).unwrap().weekday())
        }).collect()
    }

    fn filter_language(self, languages: &Vec<&SongLanguage>) -> Self {
        self.into_iter().filter(|scrobble| {
            let scrobble_languages = &scrobble.track.languages;

            let mut all = Vec::new();
            for language in languages {
                all.push(scrobble_languages.contains(language));
            }

            for scrobble_language in scrobble_languages {
                all.push(languages.contains(&scrobble_language));
            }

            all.iter().all(|l| *l)
        }).collect()
    }

    fn filter_genre(self, genres: &Vec<&str>) -> Self {
        return self.into_iter().filter(|s| {
            let contains: Vec<bool> = s.track.genres.iter().map(|g| {
                genres.contains(&g.id.as_str())
            }).collect();

            contains.iter().any(|b| *b)

        }).collect();
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

impl<'a> ScrobbleWithSongExt<'a> for Vec<ScrobbleWithSong<'a>> {
    fn get_all_languages(&self) -> HashMap<String, Vec<SongLanguage>> {
        let mut result: HashMap<String, Vec<SongLanguage>> = HashMap::new();

        for scrobble in self {
            let languages_str = match scrobble.track.languages_to_str() {
                Some(v) => v,
                None => continue
            };

            match result.get_mut(&languages_str) {
                Some(_) => {}
                None => {result.insert(languages_str, scrobble.track.languages.clone());}
            }
        }

        return result;
    }
}

impl Scrobble {
    pub fn date_time<T>(&self, tz: T) -> Option<DateTime<T>>
    where T: TimeZone {
        return Some(DateTime::from_timestamp_secs(self.submission_time as i64)?.with_timezone(&tz));
    }
}
