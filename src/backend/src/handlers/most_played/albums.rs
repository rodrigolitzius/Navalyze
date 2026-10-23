use crate::handlers::most_played::prelude::*;
use crate::analysis::albums::AlbumStat;

pub async fn most_played_albums(
    SessionExtractor(session): SessionExtractor,
    RangeWithDefault(range): RangeWithDefault,
    ResponseFilterWithDefault(response_filter): ResponseFilterWithDefault,
    weekdays: Option<WeekdaysAndTimezone>
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let mut scrobbles = navidrome.get_library().await?
        .filter_range(range);

    if let Some(days) = weekdays {
        scrobbles = scrobbles.filter_weekdays(days.set, days.timezone)
    }

    let album_stat = AlbumStat::group(&scrobbles);

    let mut all_albums: Vec<AlbumStat> = album_stat.into_values().collect();

    all_albums.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = response_filter.select(all_albums);

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
