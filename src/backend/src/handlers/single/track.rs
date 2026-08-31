use axum::extract::{Path, Json};
use serde_json::json;

use crate::{
    api::error::ApiError,
    handlers::{extract::{HandlerParams, SessionExtractor}},
    navidrome::interface::scrobble::{ScrobbleWithSongFilter},
    analysis::tracks::TrackStat
};

pub async fn track_info(
    Path(id): Path<String>,
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let scrobbles = navidrome.get_library().await?
        .filter_range(params.range)
        .filter_track(&Vec::from([id.as_str()]));

    let timestamps: Vec<u64> = scrobbles.iter().map(|s| s.scrobble.submission_time).collect();
    let timestamps = params.filter.select(&timestamps);

    let songs_stats = TrackStat::group(scrobbles);

    let mut tracks: Vec<TrackStat> = songs_stats.into_values().collect();
    tracks.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});

    if tracks.is_empty() {
        return Ok(Json(json!([])))
    }

    let response = json!({
        "name": tracks[0].name,
        "artist": tracks[0].artist,
        "album": tracks[0].album,
        "album_id": tracks[0].album_id,
        "plays": tracks[0].plays,
        "played_hours": tracks[0].played_hours,
        "timestamps": timestamps
    });

    return Ok(Json(response))
}
