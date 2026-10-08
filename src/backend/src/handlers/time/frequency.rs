use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, TimedParams, GraphParams, SessionExtractor},
    handlers::time::to_datetime_duration_vec,
    navidrome::interface::scrobble::ScrobbleWithSongFilter,
    analysis::time::frequency
};

pub async fn frequency(
    params: HandlerParams,
    timed_params: TimedParams,
    graph_params: GraphParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let scrobbles = navidrome.get_library().await?
        .filter_range(params.range)
        .filter_weekdays(&timed_params.weekdays, timed_params.tz);

    let data = to_datetime_duration_vec(scrobbles, timed_params.tz);

    let result = frequency::group(&data, graph_params.resolution);

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
