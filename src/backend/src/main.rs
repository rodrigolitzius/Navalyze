mod navidrome;
mod handlers;
mod api;
mod mbz;
mod sqlite;
mod storage;
mod analysis;
mod reqwest;

use std::{fs::File, io::Read};

use axum::{Router, routing::{get, post}};
use tower_http::{cors::{Any, CorsLayer}, services::{ServeDir}};

use crate::{
    api::{ApiState, Settings},
    handlers::{
        other::{login::*, recent::*, relay::*, stats::*, auth_check::*, art::*},
        most_played::{artists::*, albums::*, tracks::*, playlists::*},
        single::{artist::*, album::*, playlist::*, track::*},
        time::{frequency::*, artist::*, album::*, track::*, playlist::*},
    }
};

const APP_NAME: &'static str = "Navalyze";

async fn start_backend(state: ApiState, binding_address: String) {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let frontend = ServeDir::new("./dist");

    let app = Router::new()
        // Other
        .route("/api/relay/{*tail}", get(relay))
        .route("/api/art/{*id}", get(art))
        .route("/api/recent", get(recent))
        .route("/api/stats", get(stats))
        .route("/api/login", post(login))
        .route("/api/auth-check", get(auth_check))

        // most-played
        .route("/api/most-played/artists", get(most_played_artists))
        .route("/api/most-played/albums", get(most_played_albums))
        .route("/api/most-played/tracks", get(most_played_tracks))
        .route("/api/most-played/playlists", get(most_played_playlists))

        // single
        .route("/api/playlist/{*id}", get(playlist_info))
        .route("/api/artist/{*id}", get(artist_info))
        .route("/api/album/{*id}", get(album_info))
        .route("/api/track/{*id}", get(track_info))

        // time
        .route("/api/time/frequency", get(frequency))
        .route("/api/time/artist/{*id}", get(artist_time))
        .route("/api/time/album/{*id}", get(album_time))
        .route("/api/time/track/{*id}", get(track_time))
        .route("/api/time/playlist/{*id}", get(playlist_time))

        .fallback_service(frontend)
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(binding_address.clone()).await.expect("Failed to bind server");
    println!("Server listening on {}", binding_address);

    axum::serve(listener, app).await.expect("Failed to serve server");

}

#[tokio::main]
async fn main() {
    let mut toml_str = String::new();
    File::open("./settings.toml")
        .expect("Could not open settings.toml")
        .read_to_string(&mut toml_str)
        .expect("Failed to read settings.toml");

    let settings = Settings::load(toml_str.as_str()).expect("Invalid toml file");

    let mbz_session = match settings.lbz_token {
        Some(v) => Some(mbz::MbzSession::new(v)),
        None => None
    };

    let state = ApiState::new(mbz_session, settings.clone()).expect("Failed to initialize API state");

    start_backend(state, settings.bind.clone()).await;
}
