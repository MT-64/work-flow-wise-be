use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use serde::Deserialize;
use validator::Validate;

use crate::{error::ErrorResponse, AppState};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TagQuery {
    pub id: Option<String>,

    pub tag_name: Option<String>,

    pub owner_id: Option<String>,

    pub file_id: Option<String>,

    pub folder_id: Option<String>,
}

#[async_trait]
impl FromRequestParts<AppState> for TagQuery {
    type Rejection = ErrorResponse;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Query(query) = Query::<TagQuery>::from_request_parts(parts, state).await?;

        query.validate()?;

        Ok(query)
    }
}
