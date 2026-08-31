use axum::extract::{Path, Json};
use serde::Serialize;
use crate::{
    analysis::{tracks::TrackStat},
    api::error::ApiError,
    handlers::extract::{HandlerParams, SessionExtractor},
    navidrome::interface::scrobble::{ScrobbleWithSongFilter}
};

#[derive(Serialize)]
struct ResponseArtist {
    name: String,
    id: String,
}

#[derive(Serialize)]
struct Response {
    name: String,
    artists: Vec<ResponseArtist>,
    year: Option<u64>,
    tracks: Vec<TrackStat>
}

pub async fn album_info(
    Path(id): Path<String>,
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let album = navidrome.get_album(&id).await?;

    let scrobbles = navidrome.get_library().await?
        .filter_range(params.range)
        .filter_album(&Vec::from([id.as_str()]));

    let songs_stats = TrackStat::group(scrobbles);

    let mut tracks: Vec<TrackStat> = songs_stats.into_values().collect();
    tracks.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});

    let response = Response {
        name: album.name,
        artists: album.artists.into_iter().map(|a| ResponseArtist {id: a.id, name: a.name}).collect(),
        year: album.year,
        tracks: tracks
    };

    return Ok(Json(serde_json::to_value(response).unwrap()))
}
