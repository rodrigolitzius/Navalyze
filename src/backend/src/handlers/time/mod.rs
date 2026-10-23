pub mod prelude {
    pub use crate::{
        handlers::*,
        handlers::extract::{RangeWithDefault, Resolution, WeekdaysAndTimezone, Timezone, SessionExtractor},
        handlers::time::to_datetime_duration_vec,
        navidrome::interface::scrobble::ScrobbleWithSongFilter,
        analysis::time::date
    };
}

pub mod frequency;
pub mod artist;
pub mod album;
pub mod track;
pub mod playlist;
pub mod languages;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::navidrome::interface::scrobble::ScrobbleWithSong;

pub fn to_datetime_duration_vec(scrobbles_with_songs: Vec<ScrobbleWithSong>, timezone: Tz) -> Vec<(DateTime<Tz>, f64)> {
    let mut result = Vec::new();

    for scrobble in scrobbles_with_songs {
        let date_time = scrobble.scrobble.date_time(timezone).unwrap();
        let duration = scrobble.track.duration / (60.0*60.0);

        result.push((date_time, duration));
    }

    return result;
}
