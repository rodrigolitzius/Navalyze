pub mod error;

use std::{collections::HashMap, str::FromStr, sync::Arc};

use tokio::sync::RwLock;
use serde::Deserialize;
use uuid::Uuid;
use clap::*;
use toml;

use crate::{
    navidrome::interface::NavidromeInterface,
    mbz::MbzSession,
    sqlite::InternalDB,
    storage::Storage,
    api::error::SettingsError
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

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub allow_invalid_certs: bool,
    pub bind: String,
    pub lbz_token: Option<Uuid>
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

impl Settings {
    pub fn load(toml_str: &str) -> Result<Settings, SettingsError> {
        return Ok(Self::load_toml(toml_str)?.load_cli()?);
    }

    fn load_cli(mut self) -> Result<Self, SettingsError> {
        let matches = Command::new("Navalyze")
            .about("Analyses your music listening history")
            .arg(Arg::new("lbz-token").short('l').help("Your ListenBrainz token. (Optional)"))
            .arg(Arg::new("bind").short('b').help("Address the server will listen on."))
            .arg(Arg::new("allow-invalid-certs").short('c').help("Whether to allow connections to Navidrome instances without a valid SSL certificate.").action(ArgAction::SetTrue))
            .get_matches();

        if let Some(v) = matches.get_one::<String>("bind") { self.bind = v.clone(); }
        if let Some(v) = matches.get_one::<bool>("allow-invalid-certs") { self.allow_invalid_certs = v.clone(); }
        if let Some(v) = matches.get_one::<String>("lbz-token") {
            self.lbz_token = Some(Uuid::from_str(v.as_str())?);
        }

        return Ok(self);
    }

    fn load_toml(toml_str: &str) -> Result<Settings, SettingsError> {
        return Ok(toml::from_str(toml_str)?);
    }
}
