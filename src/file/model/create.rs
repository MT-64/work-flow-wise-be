use axum::{async_trait, body::Body, extract::FromRequest, http::Request};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use bytes::Bytes;
use validator::Validate;

use crate::{
    error::ErrorResponse, prisma::Visibility, validation::uuid::check_uuid_option, AppState,
};

#[derive(TryFromMultipart, Validate)]
pub struct CreateFileRequest {
    pub parent: Option<String>,

    pub visibility: Option<Visibility>,

    pub file: FieldData<Bytes>,
}

#[async_trait]
impl FromRequest<AppState, Body> for CreateFileRequest {
    type Rejection = ErrorResponse;
    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let TypedMultipart(body) =
            TypedMultipart::<CreateFileRequest>::from_request(req, state).await?;
        body.validate()?;
        Ok(body)
    }
}
