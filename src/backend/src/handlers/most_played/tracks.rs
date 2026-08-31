use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, SessionExtractor},
    analysis::tracks::TrackStat,
    navidrome::interface::scrobble::ScrobbleWithSongFilter
};

pub async fn most_played_tracks(
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let scrobbles = navidrome.get_library().await?
        .filter_range(params.range);

    let tracks_stat = TrackStat::group(scrobbles);

    let mut all_tracks: Vec<TrackStat> = tracks_stat.into_values().collect();

    all_tracks.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = params.filter.select(&all_tracks);

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
