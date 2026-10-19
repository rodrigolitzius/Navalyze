use crate::handlers::most_played::prelude::*;
use crate::analysis::artists::ArtistStat;

pub async fn most_played_artists(
    SessionExtractor(session): SessionExtractor,
    RangeWithDefault(range): RangeWithDefault,
    ResponseFilterWithDefault(response_filter): ResponseFilterWithDefault,
    ArtistRolesWithDefault(artist_roles): ArtistRolesWithDefault,
    weekdays: Option<WeekdaysAndTimezone>
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let mut scrobbles = navidrome.get_library().await?
        .filter_range(range);

    if let Some(days) = weekdays {
        scrobbles = scrobbles.filter_weekdays(days.set, days.timezone)
    }

    let artist_stat = ArtistStat::group(scrobbles, &artist_roles);

    let mut all_artists: Vec<ArtistStat> = artist_stat.into_values().collect();

    all_artists.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = response_filter.select(&all_artists);

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
