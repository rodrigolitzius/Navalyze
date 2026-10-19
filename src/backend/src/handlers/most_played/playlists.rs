use crate::handlers::most_played::prelude::*;
use crate::analysis::playlists::PlaylistStat;

pub async fn most_played_playlists(
    SessionExtractor(session): SessionExtractor,
    RangeWithDefault(range): RangeWithDefault,
    ResponseFilterWithDefault(response_filter): ResponseFilterWithDefault,
    weekdays: Option<WeekdaysAndTimezone>
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let playlists = navidrome.playlists().await?;

    let mut scrobbles = navidrome.get_library().await?
        .filter_range(range);

    if let Some(days) = weekdays {
        scrobbles = scrobbles.filter_weekdays(days.set, days.timezone)
    }

    let playlist_stats = PlaylistStat::group(scrobbles, &playlists);

    let mut response: Vec<PlaylistStat> = playlist_stats.into_iter().map(|p| p.1).collect();

    response.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = response_filter.select(&response);

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
