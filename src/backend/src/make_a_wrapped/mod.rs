pub mod error;

use reqwest::{multipart::{Form, Part}, Client, header::{HeaderMap, HeaderValue}};
use serde_json::to_string;
use bytes::Bytes;
use serde::{Serialize, Deserialize};

use crate::{
    make_a_wrapped::error::MakeAWrappedError, navidrome::interface::Image, reqwest::{ReqwestAPiErrorExt, ResponseJsonExt},
    APP_NAME
};

pub enum WrappedFormat {
    Png,
    #[allow(unused)]
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
struct PeriodData {
    label: String
}

#[derive(Serialize)]
struct WrappedData {
    artists: Vec<String>,
    tracks: Vec<String>,
    minutes: u64,
    genre: String,
    period: PeriodData
}

#[derive(Deserialize, Serialize)]
pub struct TemplatesResponse {
    pub templates: Vec<Template>
}

#[derive(Deserialize, Serialize)]
pub struct TemplateCreator {
    pub name: String,
    pub website: String
}

#[derive(Deserialize, Serialize)]
pub struct Template {
    pub creator: TemplateCreator,
    pub name: String,
    pub slug: String
}

impl MakeAWrapped {
    pub fn new(url: &str) -> Self {
        let mut default_headers = HeaderMap::new();

        default_headers.insert("X-Wrapped-Client", HeaderValue::from_static(APP_NAME));

        return Self {
            url: url.to_string(),
            client: Client::builder()
                .default_headers(default_headers)
                .build().expect("Failed to build Make a Wrapped reqwest client")
        };
    }

    pub async fn get_wrapped(
        &self,
        artists: Vec<String>,
        tracks: Vec<String>,
        minutes: u64,
        genre: &str,
        format: &WrappedFormat,
        image: Option<Image>,
        template: &str
    ) -> Result<Bytes, MakeAWrappedError> {
        let data = WrappedBody {
            format: format.to_format_param().into(),
            template: template.to_string(),
            data: WrappedData {
                artists, tracks, minutes,
                genre: genre.to_string(),
                period: PeriodData {
                    label: "This is not visible in the generated image. I think".into()
                }
            }
        };

        let payload_str = to_string(&data).map_err(|e| {
            error::MakeAWrappedError::PayloadToText(e)
        })?;

        let url = format!("{}/wrapped", self.url);

        let response = if let Some(image) = image {
            let form: Form = Form::new()
                .part("payload", Part::text(payload_str)
                    .mime_str("application/json")
                    .map_err(|e| error::MakeAWrappedError::SetMime(e))?)
                .part("artwork", Part::stream(image.bytes)
                    .file_name("gay-furry.sex") // Doesn't work without this
                    .mime_str(image.content_type.to_str()
                    .map_err(|e| error::MakeAWrappedError::ImageContentTypeToStr(e))?)
                    .map_err(|e| error::MakeAWrappedError::SetMime(e))?);

            let response = self.client.post(url)
                .multipart(form)
                .send().await
                .map_reqwest_api_err()
                .map_err(|e| {
                    error::MakeAWrappedError::Reqwest(e)
                })?;

            response
        }

        else {
            let response = self.client.post(url)
                .json(&data)
                .send().await
                .map_reqwest_api_err()
                .map_err(|e| {
                    error::MakeAWrappedError::Reqwest(e)
                })?;

            response
        };

        return Ok(response.bytes().await.map_err(|e| {error::MakeAWrappedError::ResponseToBytes(e)})?);
    }

    pub async fn get_templates(&self) -> Result<Vec<Template>, MakeAWrappedError> {
        let url = format!("{}/templates?limit=50", self.url);

        let response: TemplatesResponse = self.client
            .get(url)
            .send().await
            .map_reqwest_api_err().map_err(|e| {
                MakeAWrappedError::Reqwest(e)
            })?
            .into_json().await.map_err(|e| {
                MakeAWrappedError::Reqwest(e)
            })?;

        return Ok(response.templates);
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
