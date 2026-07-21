use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, SessionExtractor},
    navidrome::interface::scrobble::Scrobble,
    analysis::playlists::PlaylistStat
};

pub async fn most_played_playlists(
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    session.write().await.update_scrobbles().await?;
    let session = session.read().await;

    let scrobbles = session.get_scrobbles();
    let scrobbles = Scrobble::filter_range(scrobbles, params.range);

    let playlists = session.navidrome_interface.playlists().await?;

    let playlist_stats = PlaylistStat::group(scrobbles, &playlists, &session.tracks_hashmap);

    let mut response: Vec<PlaylistStat> = playlist_stats.into_iter().map(|p| p.1).collect();

    response.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = params.filter.select(&response);

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
