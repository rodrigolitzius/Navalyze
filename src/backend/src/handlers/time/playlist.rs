use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, TimedParams, SessionExtractor},
    handlers::time::to_datetime_duration_vec,
    navidrome::interface::scrobble::ScrobbleWithSongFilter,
    analysis::time::date
};

pub async fn playlist_time(
    Path(id): Path<String>,
    params: HandlerParams,
    timed_params: TimedParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let playlist = navidrome.get_playlist(&id).await?;

    let scrobbles = navidrome.get_library().await?
        .filter_range(params.range)
        .filter_track(&playlist.song_ids.iter().map(|s| s.as_str()).collect());

    let data = to_datetime_duration_vec(scrobbles, timed_params.tz);

    let result = date::group(&data, timed_params.resolution);

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
