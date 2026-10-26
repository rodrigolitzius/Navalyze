use crate::handlers::most_played::prelude::*;
use crate::analysis::distribution::languages::get_languages;

pub async fn languages(
    SessionExtractor(session): SessionExtractor,
    RangeWithDefault(range): RangeWithDefault,
    ResponseFilterWithDefault(response_filter): ResponseFilterWithDefault,
    weekdays: Option<WeekdaysAndTimezone>
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &session.read().await.navidrome_interface;

    let mut scrobbles = navidrome.get_library().await?
        .filter_range(range);

    if let Some(days) = weekdays {
        scrobbles = scrobbles.filter_weekdays(days.set, days.timezone)
    }

    let languages = get_languages(&scrobbles);

    let mut result = languages.into_iter()
        .map(|l| {(l.0, l.1)})
        .collect::<Vec<(String, f64)>>();

    result.sort_by(|a, b| {b.1.total_cmp(&a.1)});
    let result = response_filter.select(result);

    let result: HashMap<String, f64> = result.into_iter()
        .map(|l| {(l.0, l.1)}).collect();

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
