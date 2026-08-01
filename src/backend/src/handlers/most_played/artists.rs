use crate::{
    handlers::*,
    handlers::extract::{HandlerParams, SessionExtractor},
    analysis::artists::ArtistStat,
    navidrome::interface::{scrobble::Scrobble, ArtistRole},
};

pub async fn most_played_artists(
    Query(query): Query<HashMap<String, String>>,
    params: HandlerParams,
    SessionExtractor(session): SessionExtractor
) -> Result<Json<serde_json::Value>, ApiError> {
    session.write().await.update_scrobbles().await?;
    let session = session.read().await;

    let scrobbles = session.get_scrobbles();
    let scrobbles = Scrobble::filter_range(scrobbles, params.range);

    let artist_types_default = String::from("artist");
    let artist_types = query.get("type").unwrap_or(&artist_types_default);
    let artist_types: Vec<&str> = artist_types.split(",").collect();
    let artist_types = ArtistRole::from(artist_types);

    let artist_stat = ArtistStat::group(scrobbles, &session.tracks_hashmap, &artist_types);

    let mut all_artists: Vec<ArtistStat> = artist_stat.into_values().collect();

    all_artists.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = params.filter.select(&all_artists);

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
