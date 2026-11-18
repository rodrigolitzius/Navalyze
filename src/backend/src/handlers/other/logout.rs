use crate::{
    handlers::*,
    handlers::extract::SessionExtractor
};

pub async fn logout(
    SessionExtractor(session): SessionExtractor,
    State(state): State<ApiState>,
) -> Result<(), ApiError> {
    state.sessions.write().await.remove(&session.read().await.uuid);

    return Ok(());
}
