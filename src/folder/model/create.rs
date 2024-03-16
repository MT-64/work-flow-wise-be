use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::Deserialize;
use validator::Validate;

use crate::{error::ErrorResponse, prisma::Visibility, AppState};

use super::validation::check_folder_name;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderRequest {
    pub parent: Option<String>,
    pub folder_name: String,
    pub visibility: Option<Visibility>,
}

#[async_trait]
impl FromRequest<AppState, Body> for CreateFolderRequest {
    type Rejection = ErrorResponse;
    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<CreateFolderRequest>::from_request(req, state).await?;
        body.validate()?;
        Ok(body)
    }
}
