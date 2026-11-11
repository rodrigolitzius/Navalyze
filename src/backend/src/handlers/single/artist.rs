use crate::handlers::single::prelude::*;
use crate::handlers::extract::{ArtistResponseInclude, ArtistResponseType};
use crate::analysis::albums::AlbumStat;
use serde::Serialize;

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
struct ResponseArtist {
    pub name: String,
    pub id: String
}


#[derive(Serialize)]
struct ResponseTrack {
    pub id: String,
    pub name: String,
    pub artists: Vec<ResponseArtist>,
    pub album: String,
    pub album_id: String,
    pub played_hours: f64,
    pub plays: u64,
}

#[derive(Serialize)]
struct Response {
    name: String,
    artist_type: Option<String>,
    gender: Option<String>,
    albums: Vec<ResponseAlbum>,
    tracks: Vec<ResponseTrack>,
    played_hours: f64,
    plays: u64
}

pub async fn artist_info(
    SessionExtractor(session): SessionExtractor,
    State(state): State<ApiState>,
    Path(id): Path<String>,
    RangeWithDefault(range): RangeWithDefault,
    ResponseFilterWithDefault(response_filter): ResponseFilterWithDefault,
    ArtistResponseInclude(include): ArtistResponseInclude,
    weekdays: Option<WeekdaysAndTimezone>
) -> Result<Json<serde_json::Value>, ApiError> {
    let session = &session.read().await;

    let artist = session.navidrome_interface.get_artist(&id).await?;

    let mbz_artist = match artist.mbz_id {
        Some(v) => state.storage.get_artist(session.db_domain_id, v).await?,
        None => None
    };
    let mut scrobbles = session.navidrome_interface.get_library().await?
        .filter_range(range)
        .filter_artist(&HashSet::from([id.as_str()]));

    if let Some(days) = weekdays {
        scrobbles = scrobbles.filter_weekdays(days.set, days.timezone)
    }

    let (artist_type, gender) = match mbz_artist {
        Some(v) => (v.artist_type, v.gender),
        None => (None, None)
    };

    let mut played_hours = 0.0;
    let mut plays = 0;

    let mut albums_response: Option<Vec<ResponseAlbum>> = None;
    let mut tracks_response: Option<Vec<ResponseTrack>> = None;

    if include.contains(ArtistResponseType::ALBUMS) {
        albums_response = Some(
            AlbumStat::group(&scrobbles).into_values().map(|album| {
                played_hours += album.played_hours;
                plays += album.plays;

                ResponseAlbum {
                    artist: album.artist,
                    id: album.id,
                    name: album.name,
                    played_hours: album.played_hours,
                    plays: album.plays,
                    year: None
                }
            }).collect()
        );

        albums_response.as_mut().unwrap().sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    }

    if include.contains(ArtistResponseType::TRACKS) {
        tracks_response = Some(
            TrackStat::group(&scrobbles).into_values().map(|track| {
                played_hours += track.played_hours;
                plays += track.plays;

                ResponseTrack {
                    artists: track.artists.into_iter()
                        .map(|t| ResponseArtist {id: t.id, name: t.name}).collect(),
                    id: track.id,
                    name: track.name,
                    album: track.album,
                    album_id: track.album_id,
                    played_hours: track.played_hours,
                    plays: track.plays
                }
            }).collect()
        );

        tracks_response.as_mut().unwrap().sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    }

    let result = Response {
        name: artist.name,
        artist_type: artist_type,
        gender: gender,
        albums: response_filter.select(albums_response.unwrap_or(Vec::new())),
        tracks: response_filter.select(tracks_response.unwrap_or(Vec::new())),
        played_hours, plays
    };

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
