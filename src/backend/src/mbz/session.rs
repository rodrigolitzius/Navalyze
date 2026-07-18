use crate::{mbz::*, reqwest::{ReqwestAPiErrorExt, ResponseJsonExt}};

impl MbzSession {
    pub fn new(token: Uuid) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", token).as_str()).expect("Token is invalid")
        );

        default_headers.insert(
            "User-Agent",
            HeaderValue::from_str("Navalyze/0 ( https://github.com/rodrigolitzius/Navalyze )").expect("Invalid user agent string")
        );

        let client = ClientBuilder::new()
            .default_headers(default_headers)
            .build().expect("Failed to build MBZ client");

        let result = Self {
            token: token,
            client: client
        };

        return result;
    }

    pub async fn get_artist(&self, id: Uuid) -> Result<MbzArtist, MbzError>{
        let response = self.client
            .get(format!("{MBZ_URL}/metadata/artist?artist_mbids={}&inc=artist", id))
            .send()
            .await
            .map_reqwest_api_err()?;

        let artists = response.into_json::<Vec<MbzArtist>>().await?;

        artists.into_iter().next().ok_or(MbzError::EmptyArtistList)
    }
}
