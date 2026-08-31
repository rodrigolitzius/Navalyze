use serde::Serialize;

use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, SessionExtractor},
    navidrome::interface::{scrobble::ScrobbleWithSongFilter, ArtistRole}
};

#[derive(Serialize)]
struct ResponseArtist {
    name: String,
    id: String
}

#[derive(Serialize)]
struct ResponseSong {
    id: String,
    title: String,
    artist: String,
    artists: Vec<ResponseArtist>,
    album: String,
    album_id: String,
    timestamp: u64
}

pub async fn recent(
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome = &mut session.write().await.navidrome_interface;

    let mut scrobbles = navidrome.get_library().await?
        .filter_range(params.range);

    if !params.ids.is_empty() {
        scrobbles = scrobbles.filter_track(
            &params.ids.iter().map(|i| i.as_str()).collect()
        );
    }

    scrobbles.sort_by(|a, b| { b.scrobble.submission_time.cmp(&a.scrobble.submission_time)});

    let mut result: Vec<ResponseSong> = Vec::new();
    for scrobble in params.filter.select(&scrobbles) {
        let artists: Vec<ResponseArtist> = scrobble.track.artists.iter()
            .filter(|a| a.role.contains(ArtistRole::ARTIST))
            .map(|a| ResponseArtist {id: a.artist.id.clone(), name: a.artist.name.clone()})
            .collect();

        result.push(ResponseSong {
            id: scrobble.track.id.clone(),
            title: scrobble.track.title.clone(),
            artist: scrobble.track.artist.clone(),
            artists: artists,
            album: scrobble.track.album.clone(),
            album_id: scrobble.track.album_id.clone(),
            timestamp: scrobble.scrobble.submission_time
        });
    }

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
