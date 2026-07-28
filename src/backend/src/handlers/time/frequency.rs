use chrono::DateTime;
use chrono_tz::Tz;

use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, TimedParams, SessionExtractor},
    navidrome::interface::{scrobble::Scrobble},
    analysis::time::frequency
};

pub async fn frequency(
    params: HandlerParams,
    timed_params: TimedParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    session.write().await.update_scrobbles().await?;
    let session = session.read().await;

    let scrobbles = session.get_scrobbles();
    let scrobbles = Scrobble::filter_range(scrobbles, params.range);

    let mut data: Vec<(DateTime<Tz>, f64)> = Vec::new();

    for scrobble in scrobbles {
        let date_time = scrobble.date_time(timed_params.tz).unwrap();
        let duration = match session.tracks_hashmap.get(&scrobble.media_file_id) {
            Some(v) => v,
            None => continue
        }.duration / (60.0*60.0);

        data.push((date_time, duration));
    }

    let result = frequency::group(&data, timed_params.resolution);

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
