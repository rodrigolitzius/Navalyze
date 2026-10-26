use serde::Serialize;

use crate::{
    handlers::*,
    handlers::extract::{WeekdaysAndTimezone, SessionExtractor, RangeWithDefault, ResponseFilterWithDefault},
    navidrome::interface::{scrobble::ScrobbleWithSongFilter, ArtistRole}
};

#[derive(Serialize)]
struct ResponseArtist {
    name: String,
    id: String
}

#[derive(Serialize)]
struct ResponseSong {
    id: String,
    title: String,
    artist: String,
    artists: Vec<ResponseArtist>,
    album: String,
    album_id: String,
    timestamp: u64
}

pub async fn recent(
    SessionExtractor(session): SessionExtractor,
    RangeWithDefault(range): RangeWithDefault,
    ResponseFilterWithDefault(response_filter): ResponseFilterWithDefault,
    weekdays: Option<WeekdaysAndTimezone>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &session.read().await.navidrome_interface;

    let mut scrobbles = navidrome.get_library().await?
        .filter_range(range);

    if let Some(days) = weekdays {
        scrobbles = scrobbles.filter_weekdays(days.set, days.timezone)
    }

    scrobbles.sort_by(|a, b| { b.scrobble.submission_time.cmp(&a.scrobble.submission_time)});

    let mut result: Vec<ResponseSong> = Vec::new();
    for scrobble in response_filter.select(scrobbles) {
        let artists: Vec<ResponseArtist> = scrobble.track.artists.iter()
            .filter(|a| a.role.contains(ArtistRole::ARTIST))
            .map(|a| ResponseArtist {id: a.artist.id.clone(), name: a.artist.name.clone()})
            .collect();

        result.push(ResponseSong {
            id: scrobble.track.id.clone(),
            title: scrobble.track.title.clone(),
            artist: scrobble.track.artist.clone(),
            artists: artists,
            album: scrobble.track.album.clone(),
            album_id: scrobble.track.album_id.clone(),
            timestamp: scrobble.scrobble.submission_time
        });
    }

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
