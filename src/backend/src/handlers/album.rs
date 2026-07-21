use axum::extract::{Path, Json};
use serde::Serialize;
use crate::{
    analysis::{tracks::TrackStat},
    api::error::ApiError,
    handlers::extract::{HandlerParams, SessionExtractor},
    navidrome::interface::scrobble::Scrobble
};

#[derive(Serialize)]
struct Response {
    name: String,
    artist: String,
    year: Option<u64>,
    tracks: Vec<TrackStat>
}

pub async fn album_info(
    Path(id): Path<String>,
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    session.write().await.update_scrobbles().await?;
    let session = session.read().await;

    let scrobbles = session.get_scrobbles();
    let scrobbles = Scrobble::filter_range(scrobbles, params.range);
    let scrobbles = Scrobble::filter_album(scrobbles, &session.tracks_hashmap, &Vec::from([&id]));

    let album = session.navidrome_interface.get_album(&id).await?;

    let songs_stats = TrackStat::group(scrobbles, &session.tracks_hashmap);

    let mut tracks: Vec<TrackStat> = songs_stats.into_values().collect();
    tracks.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});

    let response = Response {
        name: album.name,
        artist: album.artist,
        year: album.year,
        tracks: tracks
    };

    return Ok(Json(serde_json::to_value(response).unwrap()))
}
