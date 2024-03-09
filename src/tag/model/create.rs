use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::Deserialize;
use validator::Validate;

use crate::{error::ErrorResponse, AppState};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagRequest {
    pub tag_name: String,
}

#[async_trait]
impl FromRequest<AppState, Body> for CreateTagRequest {
    type Rejection = ErrorResponse;
    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(req) = Json::<CreateTagRequest>::from_request(req, state).await?;
        Ok(req)
    }
}
