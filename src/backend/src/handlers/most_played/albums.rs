use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, SessionExtractor},
    analysis::albums::AlbumStat,
    navidrome::interface::scrobble::ScrobbleWithSongFilter
};

pub async fn most_played_albums(
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let scrobbles = navidrome.get_library().await?
        .filter_range(params.range);

    let album_stat = AlbumStat::group(scrobbles);

    let mut all_albums: Vec<AlbumStat> = album_stat.into_values().collect();

    all_albums.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = params.filter.select(&all_albums);

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
