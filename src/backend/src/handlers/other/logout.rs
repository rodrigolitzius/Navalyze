use crate::{
    handlers::*,
    handlers::extract::SessionExtractor
};

pub async fn logout(
    SessionExtractor(session): SessionExtractor,
    State(state): State<ApiState>,
) -> Result<(), ApiError> {
    state.logout(&session.read().await.uuid).await;

    return Ok(());
}
