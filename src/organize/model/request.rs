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

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizeRequest {
    pub owner_id: String,
    pub address: String,
    pub contact: String,
    pub name: String,
}

#[async_trait]
impl FromRequest<AppState, Body> for CreateOrganizeRequest {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<CreateOrganizeRequest>::from_request(req, state).await?;

        let CreateOrganizeRequest {
            owner_id,
            address,
            contact,
            name,
        } = &body;

        Ok(body)
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrganizeRequest {
    pub owner_id: Option<String>,
    pub address: Option<String>,
    pub contact: Option<String>,
    pub name: Option<String>,
}

#[async_trait]
impl FromRequest<AppState, Body> for UpdateOrganizeRequest {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<UpdateOrganizeRequest>::from_request(req, state).await?;

        let UpdateOrganizeRequest {
            owner_id,
            address,
            name,
            contact,
        } = &body;

        Ok(body)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrganizeQueryRequest {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub owner_id: Option<String>,
}

#[async_trait]
impl FromRequestParts<AppState> for OrganizeQueryRequest {
    type Rejection = ErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Query(org_query) =
            Query::<OrganizeQueryRequest>::from_request_parts(parts, state).await?;

        Ok(org_query)
    }
}
