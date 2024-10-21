use axum::{
    debug_handler,
    extract::{FromRequest, Request},
};
use loco_rs::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::views::home::HomeResponse;

#[debug_handler]
async fn current() -> Result<Response> {
    format::json(HomeResponse::new("loco"))
}

#[derive(Debug, Serialize, Deserialize)]
struct FormRequest {
    name: String,
    age: u8,
}

#[derive(Debug)]
pub enum FormOrJsonType {
    Json,
    Form,
}

#[derive(Debug)]
pub struct FormOrJson<T> {
    pub extractor_type: FormOrJsonType,
    pub extractor: T,
}

#[async_trait]
impl<S, T> FromRequest<S> for FormOrJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let content_type = req
            .headers()
            .get("content-type")
            .ok_or(Error::BadRequest(
                "Must set content-type of application/json or application/x-www-form-urlencoded."
                    .to_string(),
            ))?
            .to_str()
            .map_err(|e| {
                tracing::error!("Failed to parse content-type: {}", e);
                Error::InternalServerError
            })?;

        let parsed_data = match content_type {
            "application/json" => {
                let json: Json<T> = Json::from_request(req, state).await?;
                Ok(Self {
                    extractor_type: FormOrJsonType::Json,
                    extractor: json.0,
                })
            }
            "application/x-www-form-urlencoded" => {
                let form: Form<T> = Form::from_request(req, state).await.map_err(|e| {
                    tracing::error!("Failed to parse form data: {}", e);
                    Error::InternalServerError
                })?;
                Ok(Self {
                    extractor_type: FormOrJsonType::Form,
                    extractor: form.0,
                })
            }
            _ => Err(Error::BadRequest("Invalid header content-type".to_string())),
        };

        parsed_data
    }
}

#[debug_handler]
async fn form_handler(FormOrJson { extractor, .. }: FormOrJson<FormRequest>) -> Result<Response> {
    let x = extractor;
    Ok(Json(x).into_response())
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(current))
        .add("/", post(form_handler))
}
