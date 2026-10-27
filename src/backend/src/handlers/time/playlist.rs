use crate::handlers::time::prelude::*;

pub async fn playlist_time(
    SessionExtractor(session): SessionExtractor,
    Path(id): Path<String>,
    Timezone(timezone): Timezone,
    Resolution(resolution): Resolution,
    RangeWithDefault(range): RangeWithDefault,
    weekdays: Option<WeekdaysAndTimezone>
)  -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &session.read().await.navidrome_interface;

    let playlist = navidrome.get_playlist(&id).await.
        ok_or(ApiError::BadRequest("Unknown playlist".into()))?;

    let mut scrobbles = navidrome.get_library().await?
        .filter_range(range)
        .filter_track(&playlist.song_ids.iter().map(|s| s.as_str()).collect());

    if let Some(days) = weekdays {
        scrobbles = scrobbles.filter_weekdays(days.set, days.timezone)
    }

    let data = to_datetime_duration_vec(scrobbles, timezone);

    let result = date::group(&data, resolution);

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
