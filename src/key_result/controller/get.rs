use crate::{key_result::model::request::KrQueryRequest, prisma::key_result};
use axum::{extract::{State, Path}, routing::get, Router};
use chrono::DateTime;
use prisma_client_rust::query_core::schema_builder::constants::filters;

use crate::{
    extractors::param::ParamId,
    key_result::model::response::KeyResultResponse,
    response::WebResponse,
    state::AppState,
    WebResult,
};

#[utoipa::path(
  get,
  tag = "Key Result",
  path = "/api/v1/kr",
  params(
    ("offset" = inline(Option<i64>), Query, description = "Starting point"),
    ("limit" = inline(Option<i32>), Query, description = "Limit"),
    ("id" = inline(Option<String>), Query, description = "Kr id"),
    ("name" = inline(Option<String>), Query, description = "Kr name"),
    ("status" = inline(Option<bool>), Query, description = "status"),
    ("progress" = inline(Option<f64>), Query, description = "progress"),
    ("userId" = inline(Option<String>), Query, description = "User id"),
    ("objectiveId" = inline(Option<String>), Query, description = "Objective id"),
    ("createdAt" = inline(Option<i64>), Query, description = "Objective created at"),
    ("updatedAt" = inline(Option<i64>), Query, description = "Objective updated at"),
    ("deadline" = inline(Option<i64>), Query, description = "Kr deadline"),
  ),
  responses(
    (
      status = 200,
      description = "Get krs",
      body = Vec<KeyResultResponse>,
      example = json!(
        {
          "code": 200,
          "message": "Get all objectives successfully",
          "data": [
            
          ],
          "error": ""
        }
      )
    ),
  )
)]
pub fn get_krs() -> Router<AppState> {
    async fn get_krs_handler(
        State(AppState { keyresult_service, .. }): State<AppState>,
        KrQueryRequest { offset, limit, id, name, status, progress, user_id, objective_id, deadline, created_at, updated_at }: KrQueryRequest
    ) -> WebResult {
        let offset = offset.unwrap_or(0);

        let limit = match limit {
            Some(limit) => match limit {
                0..=50 => limit,
                _ => 10,
            },
            None => 10,
        };

        let mut filters = vec![];

        if let Some(id) = id {
            filters.push(key_result::pk_kr_id::equals(id));
        }

        if let Some(name) = name {
            filters.push(key_result::name::equals(name));
        }
        if let Some(objective_id) = objective_id {
            filters.push(key_result::objective_id::equals(objective_id));
        }


        if let Some(deadline) = deadline {
            filters.push(key_result::deadline::lt(DateTime::from_timestamp(deadline, 0).unwrap().fixed_offset()))
        }

        if let Some(status) = status {
            filters.push(key_result::status::equals(status));
        }


        if let Some(created_at) = created_at {
            filters.push(key_result::created_at::gte(
                DateTime::from_timestamp(created_at, 0)
                    .unwrap()
                    .fixed_offset(),
            ));
        }
        if let Some(updated_at) = updated_at {
            filters.push(key_result::updated_at::gte(
                DateTime::from_timestamp(updated_at, 0)
                    .unwrap()
                    .fixed_offset(),
            ));
        }

        let krs: Vec<KeyResultResponse> = keyresult_service
            .get_krs(filters, offset, limit)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();
        Ok(WebResponse::ok("Get krs successfully", krs))
    }
    Router::new().route("/", get(get_krs_handler))
}

#[utoipa::path(
  get,
  tag = "Key Result",
  path = "/api/v1/kr/{kr_id}",
  params(
    ("kr_id" = String, Path, description = "Keyresult ID")
  ),
  responses(
    (
      status = 201,
      description = "Get keyresult by kr id",
      body = KeyResultResponse,
      example = json! (
        {
          "code": 200,
          "message": "Get keyresult by id successfully",
          "data": {
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn get_kr() -> Router<AppState> {
    async fn get_kr_handler(
        State(AppState { keyresult_service, .. }): State<AppState>,
        Path(kr_id): Path<String>,
    ) -> WebResult {
        let kr: KeyResultResponse = keyresult_service.get_kr_by_id(kr_id).await?.into();
        Ok(WebResponse::ok("Get keyresult by id successfully", kr))
    }
    Router::new().route("/:kr_id", get(get_kr_handler))
}
