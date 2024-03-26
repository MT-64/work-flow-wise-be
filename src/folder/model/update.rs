use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use is_empty::IsEmpty;
use serde::Deserialize;
use validator::Validate;

use crate::{error::ErrorResponse, prisma::Visibility, AppState};

use super::validation::check_folder_name_option;

#[derive(Deserialize, Validate, IsEmpty)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFolderRequest {
    pub parent: Option<String>,

    pub folder_name: Option<String>,

    pub visibility: Option<Visibility>,
}

#[async_trait]
impl FromRequest<AppState, Body> for UpdateFolderRequest {
    type Rejection = ErrorResponse;
    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<UpdateFolderRequest>::from_request(req, state).await?;

        if body.is_empty() {
            return Err(ErrorResponse::NoContent);
        }

        body.validate()?;

        Ok(body)
    }
}
