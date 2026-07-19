mod navidrome;
mod handlers;
mod api;
mod mbz;
mod sqlite;
mod storage;
mod analysis;
mod reqwest;

use axum::{Router, routing::{get, post}};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;
use clap::{Parser};

use crate::{
    handlers::{login::*, recent::*, relay::*, artists::*, albums::*, tracks::*, artist::*, album::*},
    api::{ApiState}
};

const APP_NAME: &'static str = "Navalyze";

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    mbz_token: Option<Uuid>,

    #[arg(short, long)]
    port: u16
}

async fn start_backend(state: ApiState, listen_port: u16) {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/recent", get(recent))
        .route("/relay/{*tail}", get(relay))
        .route("/most-played/artists", get(most_played_artists))
        .route("/most-played/albums", get(most_played_albums))
        .route("/most-played/tracks", get(most_played_tracks))
        .route("/artist/{*id}", get(artist_info))
        .route("/album/{*id}", get(album_info))
        .route("/login", post(login))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", listen_port)).await.expect("Failed to bind server");
    axum::serve(listener, app).await.expect("Failed to serve server");
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mbz_session = match args.mbz_token {
        Some(v) => Some(mbz::MbzSession::new(v)),
        None => None
    };

    let state = ApiState::new(mbz_session).expect("Failed to initialize API state");

    start_backend(state, args.port).await;
}
