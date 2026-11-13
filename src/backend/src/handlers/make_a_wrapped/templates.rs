use crate::{
    handlers::*,
    handlers::extract::SessionExtractor
};

pub async fn templates(
    SessionExtractor(_session): SessionExtractor,
    State(state): State<ApiState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let make_a_wrapped = (&*state.make_a_wrapped).as_ref()
        .ok_or_else(|| ApiError::BadRequest("Make A Wrapped is disabled on this server".into()))?;

    let templates = make_a_wrapped.get_templates().await?;

    return Ok(Json(serde_json::to_value(templates).unwrap()));
}
