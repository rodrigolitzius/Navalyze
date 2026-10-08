use axum::extract::{Path, Json};
use serde::Serialize;

use crate::{
    api::error::ApiError,
    handlers::extract::{HandlerParams, TimedParams, SessionExtractor},
    navidrome::interface::scrobble::{ScrobbleWithSongFilter},
    analysis::tracks::TrackStat
};

#[derive(Serialize)]
struct Response {
    name: String,
    tracks: Vec<TrackStat>
}

pub async fn playlist_info(
    Path(id): Path<String>,
    params: HandlerParams,
    timed_params: TimedParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let playlist = navidrome.get_playlist(&id).await?;

    let scrobbles = navidrome.get_library().await?
        .filter_range(params.range)
        .filter_weekdays(&timed_params.weekdays, timed_params.tz)
        .filter_track(&playlist.song_ids.iter().map(|s| s.as_str()).collect());

    let songs_stats = TrackStat::group(scrobbles);

    let mut tracks: Vec<TrackStat> = songs_stats.into_values().collect();
    tracks.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});

    let response = Response {
        name: playlist.name,
        tracks: tracks
    };

    return Ok(Json(serde_json::to_value(response).unwrap()))
}
