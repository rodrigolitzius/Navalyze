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
    let mut scrobbles = Scrobble::filter_range(scrobbles, params.range);

    scrobbles.sort_by(|a, b| { b.submission_time.cmp(&a.submission_time)});

    let mut datetimes: Vec<DateTime<Tz>> = Vec::new();

    for scrobble in scrobbles {
        let date_time = scrobble.date_time(timed_params.tz).unwrap();
        datetimes.push(date_time);
    }

    let result = frequency::group(datetimes.iter().map(|d| d).collect(), timed_params.resolution);

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
