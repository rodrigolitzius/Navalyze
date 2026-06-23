use crate::{
    handlers::*,
    navidrome::*,
};

#[derive(Serialize, Clone)]
struct TrackStat {
    name: String,
    artist: String,
    id: String,
    plays: u64,
    played_hours: f64
}

pub async fn most_played_tracks(
    State(state): State<ApiState>,
    Query(query): Query<HashMap<String, String>>,
    auth: Auth,
    range: Range<u64>
) -> Result<Json<serde_json::Value>, ApiError> {
    let session = get_session_from_uuid(&auth.uuid, &state.sessions).await?;

    let mut track_stat: HashMap<String, TrackStat> = HashMap::new();

    let scrobbles = Scrobble::filter_range(&session.scrobbles, range);

    for scrobble in scrobbles.iter() {
        let song_data = match session.tracks_hashmap.get(&scrobble.media_file_id) {
            Some(v) => v,
            None => continue
        };

        let duration_hour = song_data.duration / (60.0*60.0);

        match track_stat.get_mut(&song_data.id.clone()) {
            Some(v) => {
                (*v).plays += 1;
                (*v).played_hours += duration_hour
            },
            None => {
                track_stat.insert(
                    song_data.id.clone(),
                    TrackStat {
                        name: song_data.title.clone(),
                        artist: song_data.artist.clone(),
                        id: song_data.id.clone(),
                        plays: 1,
                        played_hours: duration_hour
                    }
                );
            }
        };
    }

    let mut limit = get_param_default(&query, "limit", track_stat.len());
    if limit > track_stat.len() {
        limit = track_stat.len() - 1
    }

    let mut all_tracks: Vec<TrackStat> = track_stat.into_values().collect();

    all_tracks.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = all_tracks[..limit].to_vec();

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
