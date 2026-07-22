pub mod error;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    mbz::MbzSession, navidrome::interface::{
        NavidromeInterface, TrackHashmap,
        scrobble::Scrobble,
        error::NavidromeSessionError
    },
    sqlite::InternalDB,
    storage::Storage,
};

pub struct LoginSession {
    pub navidrome_interface: NavidromeInterface,
    #[allow(unused)]
    pub uuid: uuid::Uuid,
    pub tracks_hashmap: TrackHashmap,
    pub db_domain_id: i64,
    scrobbles: Vec<Scrobble>
}

#[derive(Deserialize)]
#[derive(Clone)]
#[allow(unused)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub url: String
}

// Each LoginSession has it's own lock, so that functions
// can get a reference to a single session rather than all of them
pub type RwLockLoginSession = Arc<RwLock<LoginSession>>;
pub type Sessions = RwLock<HashMap<Uuid, RwLockLoginSession>>;

#[derive(Clone)]
pub struct ApiState {
    pub sessions: Arc<Sessions>,
    pub storage: Arc<Storage>
}

impl ApiState {
    pub fn new(mbz: Option<MbzSession>) -> Result<Self, rusqlite::Error> {
        let result = Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            storage: Arc::new(Storage::new(InternalDB::new("data.db".into())?, mbz))
        };

        return Ok(result);
    }
}

impl LoginSession {
    pub fn new(
        db_domain_id: i64,
        navidrome_interface: NavidromeInterface,
        scrobbles: Vec<Scrobble>,
        tracks_hashmap: TrackHashmap,
        uuid: Uuid
    ) -> Self {
        return Self {db_domain_id, navidrome_interface, scrobbles, tracks_hashmap, uuid};
    }

    pub async fn update_scrobbles(&mut self) -> Result<(), NavidromeSessionError> {
        let last_scrobble = self.scrobbles.iter().map(|s| s.submission_time).max().unwrap_or(0);

        let mut new_scrobbles = self.navidrome_interface.scrobbles(last_scrobble).await?;

        if new_scrobbles.len() > 0 {
            self.scrobbles.append(&mut new_scrobbles);
        }

        return Ok(());
    }

    pub fn get_scrobbles(&self) -> Vec<&Scrobble> {
        return Scrobble::as_ref_vec(&self.scrobbles);
    }
}
