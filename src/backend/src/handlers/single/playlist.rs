use serde::Serialize;
use crate::handlers::single::prelude::*;

#[derive(Serialize)]
struct Response {
    name: String,
    tracks: Vec<TrackStat>
}

pub async fn playlist_info(
    SessionExtractor(session): SessionExtractor,
    Path(id): Path<String>,
    RangeWithDefault(range): RangeWithDefault,
    weekdays: Option<WeekdaysAndTimezone>
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let playlist = navidrome.get_playlist(&id).await?;

    let mut scrobbles = navidrome.get_library().await?
        .filter_range(range)
        .filter_track(&playlist.song_ids.iter().map(|s| s.as_str()).collect());

    if let Some(days) = weekdays {
        scrobbles = scrobbles.filter_weekdays(days.set, days.timezone)
    }

    let songs_stats = TrackStat::group(scrobbles);

    let mut tracks: Vec<TrackStat> = songs_stats.into_values().collect();
    tracks.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});

    let response = Response {
        name: playlist.name,
        tracks: tracks
    };

    return Ok(Json(serde_json::to_value(response).unwrap()))
}
