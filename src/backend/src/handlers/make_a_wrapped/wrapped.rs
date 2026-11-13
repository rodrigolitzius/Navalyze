use axum::http::HeaderValue;
use reqwest::header;

use crate::{
    handlers::*,
    handlers::extract::{RangeWithDefault, WeekdaysAndTimezone, SessionExtractor, ArtistRolesWithDefault, MakeAWrappedTemplate},
    navidrome::interface::scrobble::ScrobbleWithSongFilter,

    analysis::{artists::ArtistStat, tracks::TrackStat, stats::Stats, genres::GenreStat},

    make_a_wrapped::WrappedFormat
};

pub async fn make_a_wrapped(
    SessionExtractor(session): SessionExtractor,
    State(state): State<ApiState>,
    RangeWithDefault(range): RangeWithDefault,
    ArtistRolesWithDefault(artist_roles): ArtistRolesWithDefault,
    MakeAWrappedTemplate(template): MakeAWrappedTemplate,
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

    let mut tracks = TrackStat::group(&scrobbles)
        .into_iter()
        .collect::<Vec<(String, TrackStat)>>();

    tracks.sort_by(|a, b| { b.1.played_hours.total_cmp(&a.1.played_hours)});

    let mut genres = GenreStat::group(&scrobbles)
        .into_iter()
        .collect::<Vec<(String, GenreStat)>>();

    genres.sort_by(|a, b| { b.1.played_hours.total_cmp(&a.1.played_hours)});

    let minutes_played = (Stats::group(&scrobbles).played_hours * 60.0) as u64;

    let wrapped_format = WrappedFormat::Png;
    let mut wrapped_image = None;

    if let Some(first_artist) = artists.first() {
        wrapped_image = Some(navidrome.get_art(
            &first_artist.0,
            Some(&"400".to_string())
        ).await?);
    }

    let artists: Vec<String> = artists.into_iter().map(|a| a.1.name).take(5).collect();
    let tracks: Vec<String> = tracks.into_iter().map(|t| t.1.name).take(5).collect();
    let genres: Vec<String> = genres.into_iter().map(|o| o.1.name).collect::<Vec<String>>();

    let artists = if artists.is_empty() {Vec::from(["No artists".to_string()])} else {artists};
    let tracks = if tracks.is_empty() {Vec::from(["No tracks".to_string()])} else {tracks};
    let genre = genres.into_iter().next().unwrap_or(String::from("No Genre"));

    let wrapped = make_a_wrapped.get_wrapped(
        artists, tracks,
        minutes_played, &genre, &wrapped_format,
        wrapped_image, &template
    ).await?;

    let mut headers = HeaderMap::new();
    headers.append(header::CONTENT_TYPE, HeaderValue::from_str(wrapped_format.to_mime()).unwrap());

    return Ok((
        StatusCode::OK,
        headers,
        wrapped
    ));
}
