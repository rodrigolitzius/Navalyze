use crate::{
    handlers::*,
};

pub async fn server_settings(
    State(state): State<ApiState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    return Ok(Json(json!({
        "static_navidrome_url": &state.settings.navidrome_url.is_some()
    })))
}
