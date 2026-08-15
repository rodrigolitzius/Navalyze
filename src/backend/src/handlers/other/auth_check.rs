use crate::{
    handlers::*,
    handlers::extract::SessionExtractor,
};

pub async fn auth_check(
    SessionExtractor(_session): SessionExtractor,
) -> Result<(), ApiError> {
    return Ok(());
}
