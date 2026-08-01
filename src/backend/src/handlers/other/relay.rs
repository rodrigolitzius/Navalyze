use crate::{
    handlers::*,
    handlers::extract::SessionExtractor,
};

pub async fn relay(
    Query(query): Query<HashMap<String, String>>,
    Path(tail): Path<String>,
    SessionExtractor(session): SessionExtractor,
    method: Method,
    mut headers: HeaderMap,
    body: Bytes,
) -> Result<(StatusCode, HeaderMap, Bytes), ApiError> {
    let mut client_queries: Vec<(String, String)> = Vec::new();
    for query in query.iter().into_iter() {
        client_queries.push((query.0.clone(), query.1.clone()));
    }

    headers.remove(header::HOST);

    let response = session.read().await.navidrome_interface.subsonic_relay(method, &tail)
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
