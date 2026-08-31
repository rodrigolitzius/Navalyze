pub mod error;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    navidrome::interface::NavidromeInterface,
    mbz::MbzSession,
    sqlite::InternalDB,
    storage::Storage,
};

pub struct LoginSession {
    pub navidrome_interface: NavidromeInterface,
    #[allow(unused)]
    pub uuid: uuid::Uuid,
    pub db_domain_id: i64
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

pub struct Settings {
    pub allow_invalid_certs: bool
}

#[derive(Clone)]
pub struct ApiState {
    pub sessions: Arc<Sessions>,
    pub storage: Arc<Storage>,
    pub settings: Arc<Settings>
}

impl ApiState {
    pub fn new(mbz: Option<MbzSession>, settings: Settings) -> Result<Self, rusqlite::Error> {
        let result = Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            storage: Arc::new(Storage::new(InternalDB::new("data.db".into())?, mbz)),
            settings: Arc::new(settings)
        };

        return Ok(result);
    }
}

impl LoginSession {
    pub fn new(
        db_domain_id: i64,
        navidrome_interface: NavidromeInterface,
        uuid: Uuid
    ) -> Self {
        return Self {db_domain_id, navidrome_interface, uuid};
    }
}
