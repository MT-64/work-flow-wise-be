use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use uuid::Uuid;

use crate::{error::ErrorResponse, state::AppState};

pub struct ParamId(pub String);

#[async_trait]
impl FromRequestParts<AppState> for ParamId {
    type Rejection = ErrorResponse;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Path(id) = Path::<Uuid>::from_request_parts(parts, state).await?;
        Ok(Self(id.to_string()))
    }
}
