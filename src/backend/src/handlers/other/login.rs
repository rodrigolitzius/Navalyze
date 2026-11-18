use std::sync::{Arc, Weak};
use tokio::time::{Duration, interval};

use crate::{
    handlers::*,
    navidrome::interface::NavidromeInterface
};

async fn update_scrobbles_background_task(login_session: Weak<RwLock<LoginSession>>, update_interval: u64) {
    let mut interval = interval(Duration::from_secs(update_interval));

    loop {
        interval.tick().await;

        let session = match login_session.upgrade() {
            Some(v) => v,
            None => return
        };

        let new_scrobbles = session.read().await.navidrome_interface.get_new_scrobbles().await;

        let new_scrobbles = match new_scrobbles {
            Ok(v) => v,
            Err(_) => {eprintln!("Failed to fetch new scrobbles"); continue;}
        };

        let new_scrobbles = match new_scrobbles {
            Some(v) => v,
            None => continue
        };

        match session.write().await.navidrome_interface.apply_new_scrobbles(new_scrobbles).await {
            Ok(_) => {}
            Err(_) => println!("Failed to apply new scrobbles")
        }
    }
}

pub async fn login(
    State(state): State<ApiState>,
    Json(login_request): Json<RawLoginRequest>
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut login_request: LoginRequest = login_request.into();

    if let Some(static_navidrome_url) = &state.settings.navidrome_url {
        login_request.url = static_navidrome_url.clone();
    }

    let navidrome_interface = NavidromeInterface::new(login_request.clone(), state.settings.allow_invalid_certs).await?;

    let uuid = Uuid::new_v4();

    let db_domain_id = match state.storage.db.add_domain(login_request.url.clone()) {
        Ok(v) => v,
        Err(_) => {
            return Err(ApiError::DatabaseError("Could not add domain to database".into()));
        }
    };


    let login_session = Arc::new(RwLock::new(LoginSession::new(
        db_domain_id, navidrome_interface, uuid
    )));

    if state.settings.scrobble_update_interval > 0 {
        let login_session_copy = Arc::downgrade(&login_session);
        let scrobble_update_interval_copy = state.settings.scrobble_update_interval;

        tokio::spawn(async move {
            update_scrobbles_background_task(
                login_session_copy,
                scrobble_update_interval_copy
            ).await
        });
    }

    if (state.sessions.read().await.len() >= (state.settings.max_sessions as usize)) &&
    (state.settings.max_sessions != 0) {
        let mut existing_sessions = Vec::new();

        for session in state.sessions.read().await.iter() {
            let session = session.1.read().await;
            existing_sessions.push((session.uuid, session.created_at));
        }

        if let Some(min) = existing_sessions.into_iter().min_by_key(|s| s.1) {
            state.logout(&min.0).await;
        }
    }

    state.sessions.write().await.insert(uuid, login_session);

    return Ok(Json(json!({"id": uuid})));
}
