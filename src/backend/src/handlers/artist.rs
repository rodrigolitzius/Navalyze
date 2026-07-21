use serde::Serialize;

use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, SessionExtractor},
    analysis::{albums::AlbumStat},
    navidrome::interface::{scrobble::Scrobble}
};

#[derive(Serialize)]
struct ResponseAlbum {
    name: String,
    artist: String,
    id: String,
    plays: u64,
    played_hours: f64,
    year: Option<u64>
}

#[derive(Serialize)]
struct Response {
    name: String,
    album_count: u64,
    artist_type: Option<String>,
    gender: Option<String>,
    albums: Vec<ResponseAlbum>
}

pub async fn artist_info(
    State(state): State<ApiState>,
    Path(id): Path<String>,
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    session.write().await.update_scrobbles().await?;
    let session = session.read().await;

    let scrobbles = session.get_scrobbles();
    let scrobbles = Scrobble::filter_range(scrobbles, params.range);

    let artist = session.navidrome_interface.get_artist(&id).await?;

    let mbz_artist = match artist.mbz_id {
        Some(v) => state.storage.get_artist(session.db_domain_id, v).await?,
        None => None
    };

    let scrobbles = Scrobble::filter_artist(scrobbles, &session.tracks_hashmap, &Vec::from([&id]));

    let albums_stat = AlbumStat::group(scrobbles, &session.tracks_hashmap);

    let (artist_type, gender) = match mbz_artist {
        Some(v) => (v.artist_type, v.gender),
        None => (None, None)
    };

    let mut albums_response: Vec<ResponseAlbum> = Vec::new();
    for album in albums_stat.into_values() {
        let year = session.navidrome_interface.get_album(&album.id.clone()).await?.year;

        albums_response.push(ResponseAlbum {
            artist: album.artist,
            id: album.id,
            name: album.name,
            played_hours: album.played_hours,
            plays: album.plays,
            year: year
        });
    }

    albums_response.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});

    let result = Response {
        name: artist.name,
        artist_type: artist_type,
        gender: gender,
        album_count: albums_response.len() as u64,
        albums: albums_response
    };

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
