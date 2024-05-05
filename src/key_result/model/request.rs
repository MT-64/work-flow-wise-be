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
pub struct CreateKrRequest {
    pub name: String,
    pub description: String,
    pub user_id: String,
    pub objective_id: String,
    pub target: f64,
    pub metric: String,
    pub progress: Option<f64>,
    pub deadline: i64,
}

#[async_trait]
impl FromRequest<AppState, Body> for CreateKrRequest {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<CreateKrRequest>::from_request(req, state).await?;

        let CreateKrRequest {
            name,
            description,
            user_id,
            objective_id,
            target,
            progress,
            deadline,
            metric,
        } = &body;

        Ok(body)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KrQueryRequest {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub status: Option<bool>,
    pub progress: Option<f64>,
    pub user_id: Option<String>,
    pub objective_id: Option<String>,
    pub deadline: Option<i64>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

#[async_trait]
impl FromRequestParts<AppState> for KrQueryRequest {
    type Rejection = ErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Query(kr_query) = Query::<KrQueryRequest>::from_request_parts(parts, state).await?;

        Ok(kr_query)
    }
}

#[derive(Deserialize, Validate, IsEmpty, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateKrRequest {
    pub name: Option<String>,
    pub user_id: Option<String>,
    pub objective_id: Option<String>,
    pub description: Option<String>,
    pub target: Option<f64>,
    pub progress: Option<f64>,
    pub deadline: Option<i64>,
}

#[async_trait]
impl FromRequest<AppState, Body> for UpdateKrRequest {
    type Rejection = ErrorResponse;
    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<UpdateKrRequest>::from_request(req, state).await?;

        // Just return no content if the body is empty
        if body.is_empty() {
            return Err(ErrorResponse::NoContent);
        }

        let UpdateKrRequest {
            name,
            user_id,
            objective_id,
            description,
            target,
            progress,
            deadline,
        } = &body;

        Ok(body)
    }
}
#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddFileRequest {
    pub file_path: String,
}

#[async_trait]
impl FromRequest<AppState, Body> for AddFileRequest {
    type Rejection = ErrorResponse;
    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<AddFileRequest>::from_request(req, state).await?;

        // Just return no content if the body is empty

        let AddFileRequest { file_path } = &body;

        Ok(body)
    }
}
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct GradingKr {
    pub grade: f64,
}

#[async_trait]
impl FromRequest<AppState, Body> for GradingKr {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<GradingKr>::from_request(req, state).await?;

        let GradingKr { grade } = &body;

        Ok(body)
    }
}
