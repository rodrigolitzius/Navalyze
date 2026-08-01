use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, SessionExtractor},
    analysis::albums::AlbumStat,
    navidrome::interface::scrobble::Scrobble
};

pub async fn most_played_albums(
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    session.write().await.update_scrobbles().await?;
    let session = session.read().await;

    let scrobbles = session.get_scrobbles();
    let scrobbles = Scrobble::filter_range(scrobbles, params.range);

    let album_stat = AlbumStat::group(scrobbles, &session.tracks_hashmap);

    let mut all_albums: Vec<AlbumStat> = album_stat.into_values().collect();

    all_albums.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = params.filter.select(&all_albums);

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
