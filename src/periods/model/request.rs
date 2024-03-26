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
pub struct CreatePeriodRequest {
    pub name: String,
    pub organize_id: String,
    pub start_date: i64,
    pub end_date: i64,
}

#[async_trait]
impl FromRequest<AppState, Body> for CreatePeriodRequest {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<CreatePeriodRequest>::from_request(req, state).await?;

        let CreatePeriodRequest {
            name,
            organize_id,
            start_date,
            end_date,
        } = &body;

        Ok(body)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PeriodQueryRequest {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub id: Option<String>,
    pub organize_id: Option<String>,
    pub name: Option<String>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
}

#[async_trait]
impl FromRequestParts<AppState> for PeriodQueryRequest {
    type Rejection = ErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Query(period_query) =
            Query::<PeriodQueryRequest>::from_request_parts(parts, state).await?;

        Ok(period_query)
    }
}

#[derive(Deserialize, Validate, IsEmpty, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePeriodRequest {
    pub name: Option<String>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
}

#[async_trait]
impl FromRequest<AppState, Body> for UpdatePeriodRequest {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<UpdatePeriodRequest>::from_request(req, state).await?;

        if body.is_empty() {
            return Err(ErrorResponse::NoContent);
        }

        let UpdatePeriodRequest {
            name,
            start_date,
            end_date,
        } = &body;

        Ok(body)
    }
}
