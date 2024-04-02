use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, FromRequestParts, Query},
    http::request::Parts,
    http::Request,
    Json,
};
use is_empty::IsEmpty;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::{error::ErrorResponse, helpers::validation::validation_message, state::AppState};

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationQueryRequest {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub user_id: Option<String>,
    pub status: Option<bool>,
    pub timestamp: Option<i64>,
}

#[async_trait]
impl FromRequestParts<AppState> for NotificationQueryRequest {
    type Rejection = ErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Query(noti_query) =
            Query::<NotificationQueryRequest>::from_request_parts(parts, state).await?;

        Ok(noti_query)
    }
}
