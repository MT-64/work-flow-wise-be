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
pub struct CreateDepartmentRequest {
    pub manager_id: Option<String>,
    pub organize_id: String,
    pub name: String,
}

#[async_trait]
impl FromRequest<AppState, Body> for CreateDepartmentRequest {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<CreateDepartmentRequest>::from_request(req, state).await?;

        let CreateDepartmentRequest {
            manager_id,
            organize_id,
            name,
        } = &body;

        Ok(body)
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDepartmentRequest {
    pub manager_id: Option<String>,
    pub organize_id: Option<String>,
    pub name: Option<String>,
}

#[async_trait]
impl FromRequest<AppState, Body> for UpdateDepartmentRequest {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<UpdateDepartmentRequest>::from_request(req, state).await?;

        let UpdateDepartmentRequest {
            manager_id,
            organize_id,
            name,
        } = &body;

        Ok(body)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DepartmentQueryRequest {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub organize_id: Option<String>,
    pub manager_id: Option<String>,
}

#[async_trait]
impl FromRequestParts<AppState> for DepartmentQueryRequest {
    type Rejection = ErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Query(department_query) =
            Query::<DepartmentQueryRequest>::from_request_parts(parts, state).await?;

        Ok(department_query)
    }
}
