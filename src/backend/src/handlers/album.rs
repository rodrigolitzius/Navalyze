use axum::extract::{State, Path, Json};
use crate::{
    analysis::{tracks::TrackStat},
    api::{ApiState, error::ApiError},
    handlers::{Auth, Range, get_session_from_uuid},
    navidrome::scrobble::Scrobble
};

pub async fn album_info(
    State(state): State<ApiState>,
    Path(id): Path<String>,
    auth: Auth,
    range: Range<u64>
) -> Result<Json<serde_json::Value>, ApiError> {
    let session = get_session_from_uuid(&auth.uuid, &state.sessions).await?;

    let scrobbles = Scrobble::as_ref_vec(&session.scrobbles);
    let scrobbles = Scrobble::filter_range(scrobbles, range);
    let scrobbles = Scrobble::filter_album(scrobbles, &session.tracks_hashmap, &Vec::from([&id]));

    let songs_stats = TrackStat::group(scrobbles, &session.tracks_hashmap);

    let mut select: Vec<TrackStat> = songs_stats.into_values().collect();
    select.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});

    return Ok(Json(serde_json::to_value(select).unwrap()))
}
