use serde::Serialize;
use crate::handlers::single::prelude::*;

#[derive(Serialize)]
struct Response {
    name: String,
    tracks: Vec<TrackStat>,
    played_hours: f64,
    plays: u64
}

pub async fn playlist_info(
    SessionExtractor(session): SessionExtractor,
    Path(id): Path<String>,
    RangeWithDefault(range): RangeWithDefault,
    weekdays: Option<WeekdaysAndTimezone>
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &session.read().await.navidrome_interface;

    let playlist = navidrome.get_playlist(&id).await.
        ok_or(ApiError::BadRequest("Unknown playlist".into()))?;

    let mut scrobbles = navidrome.get_library().await?
        .filter_range(range)
        .filter_track(&playlist.song_ids.iter().map(|s| s.as_str()).collect());

    if let Some(days) = weekdays {
        scrobbles = scrobbles.filter_weekdays(days.set, days.timezone)
    }

    let songs_stats = TrackStat::group(&scrobbles);

    let mut played_hours = 0.0;
    let mut plays = 0;

    let mut tracks: Vec<TrackStat> = songs_stats.into_values().map(|t| {
        played_hours += t.played_hours;
        plays += t.plays;
        t
    }).collect();
    tracks.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});

    let response = Response {
        name: playlist.name.clone(),
        tracks: tracks,
        played_hours, plays
    };

    return Ok(Json(serde_json::to_value(response).unwrap()))
}
