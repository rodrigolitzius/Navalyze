use axum::http::HeaderValue;
use reqwest::header;

use crate::{
    handlers::*,
    handlers::extract::{RangeWithDefault, WeekdaysAndTimezone, SessionExtractor, ArtistRolesWithDefault},
    navidrome::interface::scrobble::ScrobbleWithSongFilter,

    analysis::{artists::ArtistStat, tracks::TrackStat, stats::Stats},

    make_a_wrapped::WrappedFormat
};

pub async fn make_a_wrapped(
    SessionExtractor(session): SessionExtractor,
    State(state): State<ApiState>,
    RangeWithDefault(range): RangeWithDefault,
    ArtistRolesWithDefault(artist_roles): ArtistRolesWithDefault,
    weekdays: Option<WeekdaysAndTimezone>
) -> Result<(StatusCode, HeaderMap, Bytes), ApiError> {
    let make_a_wrapped = (&*state.make_a_wrapped).as_ref()
        .ok_or_else(|| ApiError::BadRequest("Make A Wrapped is disabled on this server".into()))?;

    let navidrome = &session.read().await.navidrome_interface;

    let mut scrobbles = navidrome.get_library().await?
        .filter_range(range);

    if let Some(days) = weekdays {
        scrobbles = scrobbles.filter_weekdays(days.set, days.timezone)
    }

    let mut artists = ArtistStat::group(&scrobbles, &artist_roles)
        .into_iter()
        .collect::<Vec<(String, ArtistStat)>>();
    artists.sort_by(|a, b| { b.1.played_hours.total_cmp(&a.1.played_hours)});
    let artists = artists.into_iter().map(|a| a.1.name).take(3).collect();

    let mut tracks = TrackStat::group(&scrobbles)
        .into_iter()
        .collect::<Vec<(String, TrackStat)>>();
    tracks.sort_by(|a, b| { b.1.played_hours.total_cmp(&a.1.played_hours)});
    let tracks = tracks.into_iter().map(|t| t.1.name).take(3).collect();

    let minutes_played = (Stats::group(&scrobbles).played_hours * 60.0) as u64;

    let wrapped_format = WrappedFormat::Svg;
    let wrapped = make_a_wrapped.get_wrapped(artists, tracks, minutes_played, "aaaa".into(), &wrapped_format).await.map_err(|e|
        ApiError::Internal(format!("Failed to generate wrapped: {:?}", e))
    )?;

    let mut headers = HeaderMap::new();
    headers.append(header::CONTENT_TYPE, HeaderValue::from_str(wrapped_format.to_mime()).unwrap());

    return Ok((
        StatusCode::OK,
        headers,
        wrapped
    ));
}
