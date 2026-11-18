use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    handlers::*,
    handlers::extract::SessionExtractor,
};

pub async fn update_auth(
    SessionExtractor(session): SessionExtractor,
    State(state): State<ApiState>,
) -> Result<(), ApiError> {
    let session = session.read().await;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;

    if !session.navidrome_interface.token_valid(now).await {
        state.logout(&session.uuid).await;
        return Err(ApiError::Unauthorized("The Navidrome token has expired".into()));
    }

    return Ok(());
}
