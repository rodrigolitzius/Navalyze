use crate::{
    analysis::time::group_type,
    handlers::time::prelude::*
};

pub async fn album_time(
    SessionExtractor(session): SessionExtractor,
    Path(id): Path<String>,
    Timezone(timezone): Timezone,
    RangeWithDefault(range): RangeWithDefault,
    graph_type: GraphTypeAndResolution,
    weekdays: Option<WeekdaysAndTimezone>
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &session.read().await.navidrome_interface;

    let mut scrobbles = navidrome.get_library().await?
        .filter_range(range)
        .filter_album(&HashSet::from([id.as_str()]));

    if let Some(days) = weekdays {
        scrobbles = scrobbles.filter_weekdays(days.set, days.timezone)
    }

    let data = to_datetime_duration_vec(scrobbles, timezone);

    let result = group_type(&data, graph_type.resolution, &graph_type.graph_type);

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
