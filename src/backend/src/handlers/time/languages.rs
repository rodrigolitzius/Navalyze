use crate::handlers::time::prelude::*;
use crate::navidrome::interface::scrobble::ScrobbleWithSongExt;
use crate::handlers::extract::ResponseFilterWithDefault;

pub async fn languages_time(
    SessionExtractor(session): SessionExtractor,
    Timezone(timezone): Timezone,
    Resolution(resolution): Resolution,
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

    let mut all_languages: HashMap<String, HashMap<u64, f64>> = HashMap::new();

    for (language_str, languages) in scrobbles.get_all_languages().into_iter() {
        let scrobbles_copy = scrobbles.clone()
            .filter_language(&languages.iter().map(|s| s).collect());

        let data = to_datetime_duration_vec(scrobbles_copy, timezone);

        all_languages.insert(language_str, date::group(&data, resolution));
    }

    let mut sorted_languages: Vec<(String, HashMap<u64, f64>)> = all_languages.into_iter().collect();
    sorted_languages.sort_by(|a, b| {
        let hashmap_a = &a.1;
        let hashmap_b = &b.1;

        let b_value: f64 = hashmap_b.values().sum::<f64>();
        let a_value: f64 = hashmap_a.values().sum::<f64>();

        b_value.total_cmp(&a_value)
    });
    let sorted_languages = response_filter.select(sorted_languages);

    let result: HashMap<String, HashMap<u64, f64>> = sorted_languages.into_iter().map(|l| {
        (l.0, l.1)
    }).collect();

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
