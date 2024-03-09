use std::collections::HashSet;

use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::Deserialize;
use validator::Validate;

use crate::{error::ErrorResponse, AppState};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetFolderCollabRequest {
    pub folder_id: String,

    pub user_ids: HashSet<String>,
}

#[async_trait]
impl FromRequest<AppState, Body> for SetFolderCollabRequest {
    type Rejection = ErrorResponse;
    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(req) = Json::<SetFolderCollabRequest>::from_request(req, state).await?;

        req.validate()?;

        Ok(req)
    }
}
