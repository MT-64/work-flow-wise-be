use std::collections::HashSet;

use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::Deserialize;
use validator::Validate;

use crate::{error::ErrorResponse, validation::uuid::check_uuid_set, AppState};

#[derive(Deserialize, Validate)]
pub struct SetTagRequest {
    #[validate(custom(function = "check_uuid_set"))]
    pub tag_names: HashSet<String>,

    #[validate(custom(function = "check_uuid_set"))]
    pub file_ids: HashSet<String>,

    #[validate(custom(function = "check_uuid_set"))]
    pub folder_ids: HashSet<String>,
}

#[async_trait]
impl FromRequest<AppState, Body> for SetTagRequest {
    type Rejection = ErrorResponse;
    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(req) = Json::<SetTagRequest>::from_request(req, state).await?;

        req.validate()?;

        Ok(req)
    }
}
