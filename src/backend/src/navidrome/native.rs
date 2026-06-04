use crate::{navidrome::*};

impl NavidromeNativeSession {
    pub async fn new(login_request: LoginRequest) -> Result<Self, NavidromeSessionError> {
        let client = match Client::builder().tls_danger_accept_invalid_certs(true).build() {
            Ok(v) => v,
            Err(e) => {
                return Err(NavidromeSessionError::Reqwest(e));
            }
        };

        let mut body = HashMap::new();
        body.insert("username", login_request.username);
        body.insert("password", login_request.password);

        let response = client
            .post(format!("{}/auth/login", login_request.url))
            .json(&body)
            .send()
            .await;

        let response = validate_login_response(response)?;

        let login_response: LoginResponse = response
            .json()
            .await
            .expect("Required fields are missing from Navidrome's response");

        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            "x-nd-authorization",
            HeaderValue::from_str(format!("Bearer {}", login_response.token).as_str()).expect("Navidrome returned an invalid token")
        );

        let client = Client::builder()
            .default_headers(default_headers)
            .build()
            .unwrap();

        return Ok(Self {
            url: login_request.url.to_string(),
            user_id: login_response.id,
            client: client,
            token: login_response.token,
        });
    }

    pub async fn song(self: &Self, id: &str) -> Result<SongData, reqwest::Error> {
        let response = self.client.get(format!("{}/api/song/{}", self.url, id))
            .send()
            .await?
            .error_for_status()?;

        let json: SongData = response.json::<SongData>().await?;

        return Ok(json);
    }

    pub async fn build_track_hashmap(&self, scrobbles: &Vec<Scrobble>) -> HashMap<String, SongData> {
        let mut result = HashMap::new();

        for scrobble in scrobbles {
            if result.contains_key(&scrobble.media_file_id) {continue;}

            let song = self.song(&scrobble.media_file_id).await;
            if let Err(_) = song {continue;}

            result.insert(scrobble.media_file_id.clone(), song.unwrap());
        }

        return result;
    }
}
