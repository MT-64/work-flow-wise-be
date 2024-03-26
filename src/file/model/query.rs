use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use chrono::{DateTime, FixedOffset};
use is_empty::IsEmpty;
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::ErrorResponse,
    prisma::{Extension, Visibility},
    AppState,
};

#[derive(Deserialize, Validate, IsEmpty)]
#[serde(rename_all = "camelCase")]
pub struct FileQuery {
    pub id: Option<String>,

    pub owner_id: Option<String>, // ignored

    pub parent_folder_id: Option<String>, // ignored
    pub filename: Option<String>,

    pub extension: Option<Extension>,

    pub visibility: Option<Visibility>, // ignored

    pub created_at: Option<DateTime<FixedOffset>>,

    pub updated_at: Option<DateTime<FixedOffset>>,
}

#[async_trait]
impl FromRequestParts<AppState> for FileQuery {
    type Rejection = ErrorResponse;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Query(query) = Query::<FileQuery>::from_request_parts(parts, state).await?;

        Ok(query)
    }
}
