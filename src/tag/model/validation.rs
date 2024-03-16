use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use is_empty::IsEmpty;
use serde::Deserialize;
use validator::Validate;

use crate::{error::ErrorResponse, AppState};

#[derive(Deserialize, IsEmpty, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTagRequest {
    pub tag_name: Option<String>,
}

#[async_trait]
impl FromRequest<AppState, Body> for UpdateTagRequest {
    type Rejection = ErrorResponse;
    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(req) = Json::<UpdateTagRequest>::from_request(req, state).await?;

        if req.is_empty() {
            return Err(ErrorResponse::NoContent);
        }

        Ok(req)
    }
}
