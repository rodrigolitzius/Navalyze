use crate::{
    handlers::*,
    handlers::extract::{WeekdaysAndTimezone, SessionExtractor, RangeWithDefault},
    analysis::stats::Stats,
    navidrome::interface::scrobble::ScrobbleWithSongFilter
};

pub async fn stats(
    SessionExtractor(session): SessionExtractor,
    RangeWithDefault(range): RangeWithDefault,
    weekdays: Option<WeekdaysAndTimezone>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let mut scrobbles = navidrome.get_library().await?
        .filter_range(range);

    if let Some(days) = weekdays {
        scrobbles = scrobbles.filter_weekdays(days.set, days.timezone)
    }

    let stats = Stats::group(scrobbles);

    return Ok(Json(serde_json::to_value(stats).unwrap()))
}
