use axum::http::HeaderValue;
use reqwest::header;

use crate::{
    handlers::{extract::SessionExtractor, *}
};

pub async fn art(
    Query(query): Query<HashMap<String, String>>,
    Path(id): Path<String>,
    SessionExtractor(session): SessionExtractor,
) -> Result<(StatusCode, HeaderMap, Bytes), ApiError> {
    let image = session.read().await.navidrome_interface.get_art(&id, query.get("size")).await?;

    let mut headers = HeaderMap::new();
    headers.append(header::CONTENT_TYPE, image.content_type);
    headers.append(header::CACHE_CONTROL, HeaderValue::from_str("max-age=10800").unwrap());

    return Ok((StatusCode::OK, headers, image.bytes));
}
