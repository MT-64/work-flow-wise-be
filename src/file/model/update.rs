use axum::{async_trait, body::Body, extract::FromRequest, http::Request};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use bytes::Bytes;
use is_empty::IsEmpty;
use validator::Validate;

use crate::{
    error::ErrorResponse, prisma::Visibility, validation::uuid::check_uuid_option, AppState,
};

#[derive(TryFromMultipart, Validate, IsEmpty)]
pub struct UpdateFileRequest {
    pub parent: Option<String>,

    pub visibility: Option<Visibility>,

    pub file: Option<FieldData<Bytes>>,
}

#[async_trait]
impl FromRequest<AppState, Body> for UpdateFileRequest {
    type Rejection = ErrorResponse;
    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let TypedMultipart(body) =
            TypedMultipart::<UpdateFileRequest>::from_request(req, state).await?;
        if body.is_empty() {
            return Err(ErrorResponse::NoContent);
        }
        body.validate()?;
        Ok(body)
    }
}
