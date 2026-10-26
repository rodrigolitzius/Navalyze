use crate::handlers::time::prelude::*;
use crate::analysis::time::frequency;

pub async fn frequency(
    SessionExtractor(session): SessionExtractor,
    Timezone(timezone): Timezone,
    Resolution(resolution): Resolution,
    RangeWithDefault(range): RangeWithDefault,
    weekdays: Option<WeekdaysAndTimezone>
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &session.read().await.navidrome_interface;

    let mut scrobbles = navidrome.get_library().await?
        .filter_range(range);

    if let Some(days) = weekdays {
        scrobbles = scrobbles.filter_weekdays(days.set, days.timezone)
    }

    let data = to_datetime_duration_vec(scrobbles, timezone);

    let result = frequency::group(&data, resolution);

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
