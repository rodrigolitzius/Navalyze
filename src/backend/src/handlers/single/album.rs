use crate::handlers::single::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct ResponseArtist {
    name: String,
    id: String,
}

#[derive(Serialize)]
struct Response {
    name: String,
    artists: Vec<ResponseArtist>,
    year: Option<u64>,
    tracks: Vec<TrackStat>
}

pub async fn album_info(
    SessionExtractor(session): SessionExtractor,
    Path(id): Path<String>,
    RangeWithDefault(range): RangeWithDefault,
    weekdays: Option<WeekdaysAndTimezone>
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &session.read().await.navidrome_interface;

    let album = navidrome.get_album(&id).await?;

    let mut scrobbles = navidrome.get_library().await?
        .filter_range(range)
        .filter_album(&Vec::from([id.as_str()]));

    if let Some(days) = weekdays {
        scrobbles = scrobbles.filter_weekdays(days.set, days.timezone)
    }

    let songs_stats = TrackStat::group(&scrobbles);

    let mut tracks: Vec<TrackStat> = songs_stats.into_values().collect();
    tracks.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});

    let response = Response {
        name: album.name,
        artists: album.artists.into_iter().map(|a| ResponseArtist {id: a.id, name: a.name}).collect(),
        year: album.year,
        tracks: tracks
    };

    return Ok(Json(serde_json::to_value(response).unwrap()))
}
