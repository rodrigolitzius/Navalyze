use crate::handlers::time::prelude::*;
use crate::handlers::extract::ResponseFilterWithDefault;
use crate::analysis::genres::GenreStat;

pub async fn genres_time(
    SessionExtractor(session): SessionExtractor,
    Timezone(timezone): Timezone,
    Resolution(resolution): Resolution,
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

    let mut all_genres: HashMap<String, HashMap<u64, f64>> = HashMap::new();

    for (genre_id, genre_stat) in GenreStat::group(&scrobbles) {
        let scrobbles_copy = scrobbles.clone()
            .filter_genre(&Vec::from([genre_id.as_str()]));

        let data = to_datetime_duration_vec(scrobbles_copy, timezone);

        all_genres.insert(genre_stat.name, date::group(&data, resolution));
    }

    let mut sorted_genres: Vec<(String, HashMap<u64, f64>)> = all_genres.into_iter().collect();
    sorted_genres.sort_by(|a, b| {
        let hashmap_a = &a.1;
        let hashmap_b = &b.1;

        let b_value: f64 = hashmap_b.values().sum::<f64>();
        let a_value: f64 = hashmap_a.values().sum::<f64>();

        b_value.total_cmp(&a_value)
    });
    let sorted_genres = response_filter.select(sorted_genres);

    let result: HashMap<String, HashMap<u64, f64>> = sorted_genres.into_iter().map(|l| {
        (l.0, l.1)
    }).collect();

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
