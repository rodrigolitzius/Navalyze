use axum::extract::{Path, Json};
use serde_json::json;
use crate::{
    analysis::tracks::TrackStat, api::error::ApiError, handlers::{extract::{HandlerParams, SessionExtractor}}, navidrome::interface::scrobble::Scrobble
};

pub async fn track_info(
    Path(id): Path<String>,
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    session.write().await.update_scrobbles().await?;
    let session = session.read().await;

    let scrobbles = session.get_scrobbles();
    let scrobbles = Scrobble::filter_range(scrobbles, params.range);
    let scrobbles = Scrobble::filter_track(scrobbles, &session.tracks_hashmap, &Vec::from([&id]));

    let timestamps: Vec<u64> = scrobbles.iter().map(|s| s.submission_time).collect();
    let timestamps = params.filter.select(&timestamps);

    let songs_stats = TrackStat::group(scrobbles, &session.tracks_hashmap);

    let mut tracks: Vec<TrackStat> = songs_stats.into_values().collect();
    tracks.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});

    if tracks.is_empty() {
        return Ok(Json(json!([])))
    }

    let response = json!({
        "name": tracks[0].name,
        "artist": tracks[0].artist,
        "artist_id": tracks[0].artist_id,
        "album": tracks[0].album,
        "album_id": tracks[0].album_id,
        "plays": tracks[0].plays,
        "played_hours": tracks[0].played_hours,
        "timestamps": timestamps
    });

    return Ok(Json(response))
}
