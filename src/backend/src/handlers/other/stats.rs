use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, SessionExtractor},
    analysis::stats::Stats,
    navidrome::interface::scrobble::ScrobbleWithSongFilter
};

pub async fn stats(
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let scrobbles = navidrome.get_library().await?
        .filter_range(params.range);

    let stats = Stats::group(scrobbles);

    return Ok(Json(serde_json::to_value(stats).unwrap()))
}
