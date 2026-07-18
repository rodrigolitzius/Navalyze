use crate::{
    handlers::*,
    navidrome::{interface::{
        scrobble::Scrobble,
        NavidromeInterface
    }}
};

pub async fn login(
    State(state): State<ApiState>,
    Json(login_request): Json<RawLoginRequest>
) -> Result<Json<serde_json::Value>, ApiError> {
    let login_request: LoginRequest = login_request.into();

    let navidrome_interface = NavidromeInterface::new(login_request.clone()).await?;

    let scrobbles: Vec<Scrobble> = navidrome_interface.scrobbles().await?;

    let tracks_hashmap = navidrome_interface.build_track_hashmap(&scrobbles).await?;
    let uuid = Uuid::new_v4();

    let db_domain_id = match state.storage.db.add_domain(login_request.url.clone()) {
        Ok(v) => v,
        Err(_) => {
            return Err(ApiError::DatabaseError("Could not add domain to database".into()));
        }
    };

    let login_session = LoginSession {
        navidrome_interface, tracks_hashmap, uuid, scrobbles, db_domain_id
    };

    state.sessions.write().await.insert(login_session.uuid, login_session);

    return Ok(Json(json!({"id": uuid})));
}
