use crate::handlers::time::prelude::*;
use crate::handlers::extract::ResponseFilterWithDefault;
use crate::analysis::distribution::release_types::get_release_types;

pub async fn release_types_time(
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

    let mut all_release_types: HashMap<String, HashMap<u64, f64>> = HashMap::new();

    for release_type in get_release_types(&scrobbles).into_keys() {
        let scrobbles_copy = scrobbles.clone()
            .filter_release_types(&Vec::from(&[release_type.as_str()]));

        let data = to_datetime_duration_vec(scrobbles_copy, timezone);

        all_release_types.insert(release_type, date::group(&data, resolution));
    }

    let mut sorted_release_types: Vec<(String, HashMap<u64, f64>)> = all_release_types.into_iter().collect();
    sorted_release_types.sort_by(|a, b| {
        let hashmap_a = &a.1;
        let hashmap_b = &b.1;

        let b_value: f64 = hashmap_b.values().sum::<f64>();
        let a_value: f64 = hashmap_a.values().sum::<f64>();

        b_value.total_cmp(&a_value)
    });
    let sorted_release_types = response_filter.select(sorted_release_types);

    let result: HashMap<String, HashMap<u64, f64>> = sorted_release_types.into_iter().map(|l| {
        (l.0, l.1)
    }).collect();

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
