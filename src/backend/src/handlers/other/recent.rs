use serde::Serialize;

use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, SessionExtractor},
    navidrome::interface::{scrobble::Scrobble, ArtistRole}
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
    session.write().await.update_scrobbles().await?;
    let session = session.read().await;

    let scrobbles = session.get_scrobbles();
    let mut scrobbles = Scrobble::filter_range(scrobbles, params.range);

    if !params.ids.is_empty() {
        scrobbles = Scrobble::filter_track(scrobbles, &session.tracks_hashmap, &params.ids.iter().map(|i| i).collect());
    }

    scrobbles.sort_by(|a, b| { b.submission_time.cmp(&a.submission_time)});

    let mut result: Vec<ResponseSong> = Vec::new();
    for scrobble in params.filter.select(&scrobbles) {
        let music_info = match session.tracks_hashmap.get(&scrobble.media_file_id) {
            Some(v) => v,
            None => {continue;}
        };

        let artists: Vec<ResponseArtist> = music_info.artists.iter()
            .filter(|a| a.role.contains(ArtistRole::ARTIST))
            .map(|a| ResponseArtist {id: a.artist.id.clone(), name: a.artist.name.clone()})
            .collect();

        result.push(ResponseSong {
            id: music_info.id.clone(),
            title: music_info.title.clone(),
            artist: music_info.artist.clone(),
            artists: artists,
            album: music_info.album.clone(),
            album_id: music_info.album_id.clone(),
            timestamp: scrobble.submission_time
        });
    }

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
