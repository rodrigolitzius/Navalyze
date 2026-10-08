use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, TimedParams, SessionExtractor},
    analysis::stats::Stats,
    navidrome::interface::scrobble::ScrobbleWithSongFilter
};

pub async fn stats(
    params: HandlerParams,
    timed_params: TimedParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let scrobbles = navidrome.get_library().await?
        .filter_weekdays(&timed_params.weekdays, timed_params.tz)
        .filter_range(params.range);

    let stats = Stats::group(scrobbles);

    return Ok(Json(serde_json::to_value(stats).unwrap()))
}
