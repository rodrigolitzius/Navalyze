pub mod prelude {
    pub use crate::{
        handlers::*,
        handlers::extract::{RangeWithDefault, ResponseFilterWithDefault, WeekdaysAndTimezone, SessionExtractor, ArtistRolesWithDefault},
        navidrome::interface::scrobble::ScrobbleWithSongFilter
    };
}

pub mod tracks;
pub mod albums;
pub mod artists;
pub mod playlists;
pub mod genres;
