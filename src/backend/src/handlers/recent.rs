use crate::{handlers::*};

pub async fn recent(
    Query(query): Query<HashMap<String, String>>,
    State(state): State<ApiState>,
    auth: Auth
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut limit = get_param_default(&query, "limit", 0) as usize;
    let offset = get_param_default(&query, "offset", 0) as usize;

    if limit == 0 {
        limit = state.scrobbles.len();
    }

    let session = state.sessions.read().await;
    let session = match session.get(&auth.uuid) {
        Some(v) => v,
        None => {
            return Err(ApiError::Internal("Could not find token".into()));
        }
    };

    let mut result: Vec<serde_json::Value> = Vec::new();
    for scrobble in session.scrobbles.iter().skip(offset).take(limit) {
        let music_info = match session.tracks_hashmap.get(&scrobble.media_file_id) {
            Some(v) => v,
            None => {continue;}
        };

        result.push(json!({
            "title": music_info["title"],
            "artist": music_info["artist"],
            "album": music_info["album"],
        }));
    }

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
