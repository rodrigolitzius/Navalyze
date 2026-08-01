use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, SessionExtractor},
    navidrome::interface::scrobble::Scrobble
};

pub async fn recent(
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    session.write().await.update_scrobbles().await?;
    let session = session.read().await;

    let scrobbles = session.get_scrobbles();
    let mut scrobbles = Scrobble::filter_range(scrobbles, params.range);

    scrobbles.sort_by(|a, b| { b.submission_time.cmp(&a.submission_time)});

    let mut result: Vec<serde_json::Value> = Vec::new();
    for scrobble in params.filter.select(&scrobbles) {
        let music_info = match session.tracks_hashmap.get(&scrobble.media_file_id) {
            Some(v) => v,
            None => {continue;}
        };

        result.push(json!({
            "id": music_info.id,
            "title": music_info.title,
            "artist": music_info.artist,
            "album": music_info.album,
        }));
    }

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
