use crate::{navidrome::*};

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

        let response = reqwest::Client::new()
            .request(Method::GET, format!("{}/rest/ping", login_request.url))
            .query(&default_params)
            .send()
            .await;

        let _response = validate_login_response(response)?;

        let result = Self {
            default_params: default_params,
            client: reqwest::Client::new(),
            salt: salt,
            token: hash
        };

        return Ok(result)
    }
}
