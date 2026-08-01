use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, SessionExtractor},
    analysis::tracks::TrackStat,
    navidrome::interface::scrobble::Scrobble
};

pub async fn most_played_tracks(
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    session.write().await.update_scrobbles().await?;
    let session = session.read().await;

    let scrobbles = session.get_scrobbles();
    let scrobbles = Scrobble::filter_range(scrobbles, params.range);

    let tracks_stat = TrackStat::group(scrobbles, &session.tracks_hashmap);

    let mut all_tracks: Vec<TrackStat> = tracks_stat.into_values().collect();

    all_tracks.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = params.filter.select(&all_tracks);

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
