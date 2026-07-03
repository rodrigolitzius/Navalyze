use rand::{RngExt, distr::Alphanumeric};

use reqwest::{Client, Method};

use crate::{
    navidrome::{
        subsonic::{SubsonicResponseArtist, SubsonicResponseArtistField, NavidromeSubsonicSession},
        SubsonicResponse,
        NavidromeSessionError
    },
    handlers::LoginRequest,
    reqwest::{ReqwestAPiErrorExt, ResponseJsonExt}
};

impl NavidromeSubsonicSession {
    // TODO: Actually test if this fails if the login request has invalid credentials
    pub async fn new(login_request: LoginRequest) -> Result<Self, NavidromeSessionError> {
        let salt: String = rand::rng()
            .sample_iter(Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        let hash = format!("{:x}", md5::compute(format!("{}{}", login_request.password, salt)));

        let mut default_params: Vec<(String, String)> = Vec::new();
        default_params.push(("u".to_string(), login_request.username));
        default_params.push(("s".to_string(), salt.clone()));
        default_params.push(("t".to_string(), hash.clone()));
        default_params.push(("c".to_string(), crate::APP_NAME.to_string()));
        default_params.push(("v".to_string(), "1.8.0".to_string()));
        default_params.push(("f".to_string(), "json".to_string()));

        let url = format!("{}/rest/ping", login_request.url);

        let client = match Client::builder().tls_danger_accept_invalid_certs(true).build() {
            Ok(v) => v,
            Err(e) => {
                return Err(NavidromeSessionError::Reqwest(e));
            }
        };

        let _response = client
            .request(Method::GET, url)
            .query(&default_params)
            .send()
            .await
            .map_reqwest_api_err()?;

        let result = Self {
            default_params: default_params,
            url: login_request.url,
            client: client
        };

        return Ok(result)
    }

    pub async fn get_artist(&self, id: &String) -> Result<SubsonicResponseArtist, NavidromeSessionError> {
        let url = format!("{}/rest/getArtist?id={}", self.url, id);

        let mut client_queries: Vec<(String, String)> = Vec::new();
        client_queries.push(("id".to_string(), id.clone()));

        let response = self.client
            .get(url)
            .query(&self.default_params)
            .query(&client_queries)
            .send()
            .await
            .map_reqwest_api_err()?;

        let artist: SubsonicResponse<SubsonicResponseArtistField> = response.into_json().await?;

        return Ok(artist.subsonic_response.artist);
    }
}
