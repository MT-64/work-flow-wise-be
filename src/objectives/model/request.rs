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
pub struct CreateObjRequest {
    pub obj_type: String,
    pub period_id: String,
    pub name: String,
    pub description: Option<String>,
    pub target: String,
    pub progress: Option<f64>,
    pub deadline: i64,
}

#[async_trait]
impl FromRequest<AppState, Body> for CreateObjRequest {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<CreateObjRequest>::from_request(req, state).await?;

        let CreateObjRequest {
            obj_type,
            period_id,
            name,
            description,
            target,
            progress,
            deadline,
        } = &body;

        Ok(body)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ObjQueryRequest {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub id: Option<String>,
    pub period_id: Option<String>,
    pub name: Option<String>,
    pub status: Option<bool>,
    pub progress: Option<f64>,
    pub obj_type: Option<String>,
    pub deadline: Option<i64>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

#[async_trait]
impl FromRequestParts<AppState> for ObjQueryRequest {
    type Rejection = ErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Query(obj_query) = Query::<ObjQueryRequest>::from_request_parts(parts, state).await?;

        Ok(obj_query)
    }
}

#[derive(Deserialize, Validate, IsEmpty, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateObjRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub period_id: Option<String>,
    pub target: Option<String>,
    pub progress: Option<f64>,
    pub deadline: Option<i64>,
}

#[async_trait]
impl FromRequest<AppState, Body> for UpdateObjRequest {
    type Rejection = ErrorResponse;
    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<UpdateObjRequest>::from_request(req, state).await?;

        // Just return no content if the body is empty
        if body.is_empty() {
            return Err(ErrorResponse::NoContent);
        }

        let UpdateObjRequest {
            name,
            period_id,
            description,
            target,
            progress,
            deadline,
        } = &body;

        Ok(body)
    }
}
