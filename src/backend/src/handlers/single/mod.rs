pub mod prelude {
    pub use crate::{
        handlers::*,
        handlers::extract::{RangeWithDefault, ResponseFilterWithDefault, WeekdaysAndTimezone, SessionExtractor},
        navidrome::interface::scrobble::ScrobbleWithSongFilter,
        analysis::tracks::TrackStat
    };
}

pub mod artist;
pub mod album;
pub mod track;
pub mod playlist;
