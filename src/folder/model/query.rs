use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use chrono::{DateTime, FixedOffset};
use is_empty::IsEmpty;
use serde::Deserialize;
use validator::Validate;

use crate::{error::ErrorResponse, prisma::Visibility, AppState};

use super::validation::check_folder_name_option;

#[derive(Deserialize, Validate, IsEmpty)]
#[serde(rename_all = "camelCase")]
pub struct FolderQuery {
    pub id: Option<String>,

    pub owner_id: Option<String>,

    pub parent_folder_id: Option<String>,

    pub folder_name: Option<String>,

    pub visibility: Option<Visibility>,

    pub created_at: Option<DateTime<FixedOffset>>,

    pub updated_at: Option<DateTime<FixedOffset>>,
}

#[async_trait]
impl FromRequestParts<AppState> for FolderQuery {
    type Rejection = ErrorResponse;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Query(query) = Query::<FolderQuery>::from_request_parts(parts, state).await?;

        Ok(query)
    }
}
