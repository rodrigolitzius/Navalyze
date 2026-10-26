use crate::handlers::single::prelude::*;

pub async fn track_info(
    SessionExtractor(session): SessionExtractor,
    Path(id): Path<String>,
    RangeWithDefault(range): RangeWithDefault,
    ResponseFilterWithDefault(response_filter): ResponseFilterWithDefault,
    weekdays: Option<WeekdaysAndTimezone>
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &session.read().await.navidrome_interface;

    let mut scrobbles = navidrome.get_library().await?
        .filter_range(range)
        .filter_track(&Vec::from([id.as_str()]));

    if let Some(days) = weekdays {
        scrobbles = scrobbles.filter_weekdays(days.set, days.timezone)
    }

    let timestamps: Vec<u64> = scrobbles.iter().map(|s| s.scrobble.submission_time).collect();
    let timestamps = response_filter.select(timestamps);

    let songs_stats = TrackStat::group(&scrobbles);

    let mut tracks: Vec<TrackStat> = songs_stats.into_values().collect();
    tracks.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});

    if tracks.is_empty() {
        return Ok(Json(json!([])))
    }

    let response = json!({
        "name": tracks[0].name,
        "artists": tracks[0].artists,
        "album": tracks[0].album,
        "album_id": tracks[0].album_id,
        "plays": tracks[0].plays,
        "played_hours": tracks[0].played_hours,
        "timestamps": timestamps
    });

    return Ok(Json(response))
}
