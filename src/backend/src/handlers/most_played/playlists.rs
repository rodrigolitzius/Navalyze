use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, SessionExtractor},
    navidrome::interface::scrobble::ScrobbleWithSongFilter,
    analysis::playlists::PlaylistStat
};

pub async fn most_played_playlists(
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let playlists = navidrome.playlists().await?;

    let scrobbles = navidrome.get_library().await?
        .filter_range(params.range);

    let playlist_stats = PlaylistStat::group(scrobbles, &playlists);

    let mut response: Vec<PlaylistStat> = playlist_stats.into_iter().map(|p| p.1).collect();

    response.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = params.filter.select(&response);

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
