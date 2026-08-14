use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, TimedParams, SessionExtractor},
    handlers::time::to_datetime_duration_vec,
    navidrome::interface::{scrobble::Scrobble},
    analysis::time::date
};

pub async fn playlist_time(
    Path(id): Path<String>,
    params: HandlerParams,
    timed_params: TimedParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    session.write().await.update_scrobbles().await?;
    let session = session.read().await;

    let playlist = session.navidrome_interface.get_playlist(&id).await?;
    let track_ids: Vec<&String> = playlist.song_ids.iter().map(|i| i).collect();

    let scrobbles = session.get_scrobbles();
    let scrobbles = Scrobble::filter_range(scrobbles, params.range);
    let scrobbles = Scrobble::filter_track(scrobbles, &session.tracks_hashmap, &track_ids);

    let data = to_datetime_duration_vec(&scrobbles, &session.tracks_hashmap, timed_params.tz);

    let result = date::group(&data, timed_params.resolution);

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
