use crate::{
    handlers::*,
    navidrome::interface::scrobble::Scrobble,
    analysis::playlists::PlaylistStat
};

pub async fn most_played_playlists(
    State(state): State<ApiState>,
    auth: Auth,
    range: Range<u64>
) -> Result<Json<serde_json::Value>, ApiError> {
    let session = get_session_from_uuid(&auth.uuid, &state.sessions).await?;

    session.write().await.update_scrobbles().await?;
    let session = session.read().await;

    let scrobbles = session.get_scrobbles();
    let scrobbles = Scrobble::filter_range(scrobbles, range);

    let playlists = session.navidrome_interface.playlists().await?;

    let playlist_stats = PlaylistStat::group(scrobbles, &playlists, &session.tracks_hashmap);

    return Ok(Json(serde_json::to_value(playlist_stats).unwrap()));
}
