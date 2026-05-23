use crate::{handlers::*};

pub async fn relay(
    Query(query): Query<HashMap<String, String>>,
    State(state): State<ApiState>,
    Path(tail): Path<String>,
    method: Method,
    headers: HeaderMap,
    auth: Auth,
    body: Bytes,
) -> Result<(StatusCode, HeaderMap, Bytes), ApiError> {
    let session = state.sessions.read().await;
    let session = match session.get(&auth.uuid) {
        Some(v) => v,
        None => {
            return Err(ApiError::Internal("Could not find token".into()));
        }
    };

    let mut client_queries: Vec<(String, String)> = Vec::new();
    for query in query.iter().into_iter() {
        client_queries.push((query.0.clone(), query.1.clone()));
    }

    let url = format!("{}/rest/{}", session.navidrome_native.url, tail);

    let response = session.navidrome_subsonic.client
        .request(method, url)
        .query(&session.navidrome_subsonic.default_params)
        .query(&client_queries)
        .headers(headers)
        .body(body)
        .send()
        .await;

    let response = match response {
        Ok(v) => v,
        Err(_) => {
            return Err(ApiError::NavidromeUnreachable("Failed to reach Navidrome".into()));
        }
    };

    let status = response.status();
    let mut headers = response.headers().clone();
    let body = response.bytes().await.unwrap();

    headers.remove(header::CONNECTION);
    headers.remove(header::PROXY_AUTHENTICATE);
    headers.remove(header::PROXY_AUTHORIZATION);
    headers.remove(header::TE);
    headers.remove(header::TRAILER);
    headers.remove(header::TRANSFER_ENCODING);
    headers.remove(header::UPGRADE);
    headers.remove("keep-alive");

    return Ok((status, headers, body))
}
