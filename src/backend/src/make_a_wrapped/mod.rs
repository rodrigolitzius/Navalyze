use reqwest::Client;
use bytes::Bytes;
use serde::Serialize;

use crate::reqwest::{ReqwestAPiErrorExt, ReqwestApiError};

pub enum WrappedFormat {
    #[allow(unused)]
    Png,
    Svg
}

pub struct MakeAWrapped {
    url: String,
    client: Client
}

#[derive(Serialize)]
struct WrappedBody {
    template: String,
    data: WrappedData,
    format: String
}

#[derive(Serialize)]
struct WrappedData {
    artists: Vec<String>,
    tracks: Vec<String>,
    minutes: u64,
    genre: String,
    period: PeriodData
}

#[derive(Serialize)]
struct PeriodData {
    label: String
}

impl MakeAWrapped {
    pub fn new(url: &str) -> Self {
        return Self {
            url: url.to_string(),
            client: Client::new()
        };
    }

    pub async fn get_wrapped(
        &self,
        artists: Vec<String>,
        tracks: Vec<String>,
        minutes: u64,
        genre: String,
        format: &WrappedFormat
    ) -> Result<Bytes, ReqwestApiError> {
        let data = WrappedBody {
            format: format.to_format_param().into(),
            template: "black".into(),
            data: WrappedData {
                artists, tracks, minutes, genre,
                period: PeriodData {
                    label: "This is not visible in the generated image. I think".into()
                }
            }
        };

        let url = format!("{}/wrapped", self.url);
        let response = self.client.post(url)
            .json(&data)
            .send()
            .await.map_reqwest_api_err()?
            .bytes().await.map_err(|e| {
                ReqwestApiError::Other(e)
            })?;

        return Ok(response);
    }
}

impl WrappedFormat {
    fn to_format_param(&self) -> &str {
        return match self {
            Self::Png => "png",
            Self::Svg => "svg"
        }
    }

    pub fn to_mime(&self) -> &str {
        match self {
            Self::Png => "image/png",
            Self::Svg => "image/svg+xml"
        }
    }
}
