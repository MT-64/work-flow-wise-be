use crate::{prisma::{self, objective::{self, obj_type}}, objectives::model::request::ObjQueryRequest};
use axum::{extract::{State, Path}, routing::get, Router};
use chrono::DateTime;
use prisma_client_rust::query_core::schema_builder::constants::filters;

use crate::{
    extractors::param::ParamId,
    objectives::model::response::ObjectiveResponse,
    response::WebResponse,
    state::AppState,
    WebResult,
};

#[utoipa::path(
  get,
  tag = "Objective",
  path = "/api/v1/objective",
  params(
    ("offset" = inline(Option<i64>), Query, description = "Starting point"),
    ("limit" = inline(Option<i32>), Query, description = "Limit"),
    ("id" = inline(Option<String>), Query, description = "Obj id"),
    ("period_id" = inline(Option<String>), Query, description = "Period id"),
    ("name" = inline(Option<String>), Query, description = "Obj name"),
    ("status" = inline(Option<bool>), Query, description = "status"),
    ("progress" = inline(Option<f64>), Query, description = "progress"),
    ("objType" = inline(Option<String>), Query, description = "objective type"),
    ("createdAt" = inline(Option<i64>), Query, description = "Objective created at"),
    ("updatedAt" = inline(Option<i64>), Query, description = "Objective updated at"),
    ("deadline" = inline(Option<i64>), Query, description = "Objective deadline"),
  ),
  responses(
    (
      status = 200,
      description = "Get objs",
      body = Vec<ObjectiveResponse>,
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
pub fn get_objs() -> Router<AppState> {
    async fn get_objs_handler(
        State(AppState { obj_service, .. }): State<AppState>,
        ObjQueryRequest {deadline,  offset, limit, id, name, status, progress, obj_type, created_at, updated_at, period_id }: ObjQueryRequest
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
            filters.push(objective::pk_objective_id::equals(id));
        }

        if let Some(name) = name {
            filters.push(objective::name::equals(name));
        }

        if let Some(deadline) = deadline {
            filters.push(objective::deadline::lt(DateTime::from_timestamp(deadline, 0).unwrap().fixed_offset()))
        }

        if let Some(status) = status {
            filters.push(objective::status::equals(status));
        }

        if let Some(period_id) = period_id {
            filters.push(objective::period_id::equals(period_id));
        }


       if let Some(obj_type) = obj_type {
            let n_obj_type = match obj_type.trim() {
            "Percent" => prisma::ObjectiveType::Percent,
            "Kpi" => prisma::ObjectiveType::Kpi,
            "As high as possible" => prisma::ObjectiveType::AsHighAsPossible,
            "As low as possible" => prisma::ObjectiveType::AsLowAsPossible,
            _ => prisma::ObjectiveType::Other,
            };

            filters.push(objective::obj_type::equals(n_obj_type));
        }
        if let Some(created_at) = created_at {
            filters.push(objective::created_at::gte(
                DateTime::from_timestamp(created_at, 0)
                    .unwrap()
                    .fixed_offset(),
            ));
        }
        if let Some(updated_at) = updated_at {
            filters.push(objective::updated_at::gte(
                DateTime::from_timestamp(updated_at, 0)
                    .unwrap()
                    .fixed_offset(),
            ));
        }

        let objs: Vec<ObjectiveResponse> = obj_service
            .get_objs(filters, offset, limit)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();
        Ok(WebResponse::ok("Get objs successfully", objs))
    }
    Router::new().route("/", get(get_objs_handler))
}

#[utoipa::path(
  get,
  tag = "Objective",
  path = "/api/v1/objective/{obj_id}",
  params(
    ("obj_id" = String, Path, description = "Objective ID")
  ),
  responses(
    (
      status = 201,
      description = "Get objective by obj id",
      body = ObjectiveResponse,
      example = json! (
        {
          "code": 200,
          "message": "Get objective by id successfully",
          "data": {
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn get_obj() -> Router<AppState> {
    async fn get_obj_handler(
        State(AppState { obj_service, .. }): State<AppState>,
        Path(obj_id): Path<String>,
    ) -> WebResult {
        let obj: ObjectiveResponse = obj_service.get_obj_by_id(obj_id).await?.into();
        Ok(WebResponse::ok("Get objective by id successfully", obj))
    }
    Router::new().route("/:obj_id", get(get_obj_handler))
}

#[utoipa::path(
  get,
  tag = "Objective",
  path = "/api/v1/objective/get_by_department/{department_id}",
  params(
    ("department_id" = String, Path, description = "departmen ID")
  ),
  responses(
    (
      status = 201,
      description = "Get objective by department id",
      body = ObjectiveResponse,
      example = json! (
        {
          "code": 200,
          "message": "Get objective by department id successfully",
          "data": {
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn get_objs_by_department() -> Router<AppState> {
    async fn get_objs_by_department_handler(
        State(AppState { obj_service, .. }): State<AppState>,
        Path(department_id): Path<String>,
    ) -> WebResult {
        let objs: Vec<ObjectiveResponse> = obj_service.
            get_objs_by_department(department_id)
            .await?.into_iter().map(|o| o.into()).collect();
        Ok(WebResponse::ok("Get objective by department id successfully", objs))
    }
    Router::new().route("/get_by_department/:department_id", get(get_objs_by_department_handler))
}

#[utoipa::path(
  get,
  tag = "Objective",
  path = "/api/v1/objective/get_by_org/{org_id}",
  params(
    ("org_id" = String, Path, description = "organize ID")
  ),
  responses(
    (
      status = 201,
      description = "Get objective by org id",
      body = ObjectiveResponse,
      example = json! (
        {
          "code": 200,
          "message": "Get objective by org id successfully",
          "data": {
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn get_objs_by_org() -> Router<AppState> {
    async fn get_objs_by_org_handler(
        State(AppState { obj_service, .. }): State<AppState>,
        Path(org_id): Path<String>,
    ) -> WebResult {
        let objs: Vec<ObjectiveResponse> = obj_service.
            get_objs_by_org(org_id)
            .await?.into_iter().map(|o| o.into()).collect();
        Ok(WebResponse::ok("Get objective by organize id successfully", objs))
    }
    Router::new().route("/get_by_org/:org_id", get(get_objs_by_org_handler))

}
#[utoipa::path(
  get,
  tag = "Objective",
  path = "/api/v1/objective/get_by_user/{user_id}",
  params(
    ("user_id" = String, Path, description = "user ID")
  ),
  responses(
    (
      status = 201,
      description = "Get objective by user id",
      body = ObjectiveResponse,
      example = json! (
        {
          "code": 200,
          "message": "Get objective by user id successfully",
          "data": {
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn get_objs_by_user() -> Router<AppState> {
    async fn get_objs_by_user_handler(
        State(AppState { obj_service, .. }): State<AppState>,
        Path(user_id): Path<String>,
    ) -> WebResult {
        let objs: Vec<ObjectiveResponse> = obj_service.
            get_objs_by_user(user_id)
            .await?.into_iter().map(|o| o.into()).collect();
        Ok(WebResponse::ok("Get objective by user id successfully", objs))
    }
    Router::new().route("/get_by_user/:user_id", get(get_objs_by_user_handler))
}
