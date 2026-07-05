use serde::Serialize;

use crate::{
    handlers::*,
    analysis::{albums::AlbumStat},
    navidrome::interface::{scrobble::Scrobble}
};

#[derive(Serialize)]
struct Response {
    name: String,
    album_count: u64,
    artist_type: Option<String>,
    gender: Option<String>,
    albums: Vec<AlbumStat>
}

pub async fn artist_info(
    State(state): State<ApiState>,
    Path(id): Path<String>,
    auth: Auth,
    range: Range<u64>
) -> Result<Json<serde_json::Value>, ApiError> {
    let session = get_session_from_uuid(&auth.uuid, &state.sessions).await?;

    let scrobbles = Scrobble::as_ref_vec(&session.scrobbles);
    let scrobbles = Scrobble::filter_range(scrobbles, range);

    let artist = session.navidrome_interface.get_artist(&id).await?;

    let mbz_artist = match artist.mbz_id {
        Some(v) => state.storage.get_artist(session.db_domain_id, v).await?,
        None => None
    };

    let scrobbles = Scrobble::filter_artist(scrobbles, &session.tracks_hashmap, &Vec::from([&id]));

    let albums_stat = AlbumStat::group(scrobbles, &session.tracks_hashmap);

    let (artist_type, gender) = match mbz_artist {
        Some(v) => (Some(v.artist_type), v.gender),
        None => (None, None)
    };

    let result = Response {
        name: artist.name,
        artist_type: artist_type,
        gender: gender,
        album_count: albums_stat.len() as u64,
        albums: albums_stat.into_values().collect()
    };

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
