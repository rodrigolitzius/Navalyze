use axum::extract::{State, Path, Json};
use serde::Serialize;

use crate::{
    api::{ApiState, error::ApiError},
    handlers::{Auth, extract::HandlerParams, get_session_from_uuid},
    navidrome::interface::scrobble::Scrobble,
    analysis::tracks::TrackStat
};

#[derive(Serialize)]
struct Response {
    name: String,
    tracks: Vec<TrackStat>
}

pub async fn playlist_info(
    State(state): State<ApiState>,
    Path(id): Path<String>,
    params: HandlerParams,
    auth: Auth,
) -> Result<Json<serde_json::Value>, ApiError> {
    let session = get_session_from_uuid(&auth.uuid, &state.sessions).await?;

    session.write().await.update_scrobbles().await?;
    let session = session.read().await;

    let playlist = session.navidrome_interface.get_playlist(&id).await?;
    let track_ids: Vec<&String> = playlist.song_ids.iter().map(|i| i).collect();

    let scrobbles = session.get_scrobbles();
    let scrobbles = Scrobble::filter_range(scrobbles, params.range);
    let scrobbles = Scrobble::filter_track(scrobbles, &session.tracks_hashmap, &track_ids);

    let songs_stats = TrackStat::group(scrobbles, &session.tracks_hashmap);

    let mut tracks: Vec<TrackStat> = songs_stats.into_values().collect();
    tracks.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});

    let response = Response {
        name: playlist.name,
        tracks: tracks
    };

    return Ok(Json(serde_json::to_value(response).unwrap()))
}
