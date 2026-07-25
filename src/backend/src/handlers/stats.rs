use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, SessionExtractor},
    analysis::stats::Stats,
    navidrome::interface::scrobble::Scrobble
};

pub async fn stats(
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    session.write().await.update_scrobbles().await?;
    let session = session.read().await;

    let scrobbles = session.get_scrobbles();
    let scrobbles = Scrobble::filter_range(scrobbles, params.range);

    let stats = Stats::group(scrobbles, &session.tracks_hashmap);

    return Ok(Json(serde_json::to_value(stats).unwrap()))
}
