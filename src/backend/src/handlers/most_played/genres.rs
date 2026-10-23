use crate::handlers::most_played::prelude::*;
use crate::analysis::genres::GenreStat;

pub async fn most_played_genres(
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

    let genre_stat = GenreStat::group(&scrobbles);

    let mut all_genres: Vec<GenreStat> = genre_stat.into_values().collect();

    all_genres.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = response_filter.select(all_genres);

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
