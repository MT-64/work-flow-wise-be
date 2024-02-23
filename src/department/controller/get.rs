use crate::department::model::request::DepartmentQueryRequest;
use axum::{extract::{State, Path}, routing::get, Router};
use chrono::DateTime;
use prisma_client_rust::query_core::schema_builder::constants::filters;
use crate::prisma::department;

use crate::{
    extractors::param::ParamId,
    department::model::response::DepartmentResponse,
    response::WebResponse,
    state::AppState,
    WebResult,
};

#[utoipa::path(
  get,
  tag = "Department",
  path = "/api/v1/department",
  params(
    ("offset" = inline(Option<i64>), Query, description = "Starting point"),
    ("limit" = inline(Option<i32>), Query, description = "Limit"),
    ("id" = inline(Option<String>), Query, description = "Obj id"),
    ("name" = inline(Option<String>), Query, description = "Obj name"),
    ("organize_id" = inline(Option<String>), Query, description = "organize id"),
    ("manager_id" = inline(Option<f64>), Query, description = "manager id"),
  ),
  responses(
    (
      status = 200,
      description = "Get objs",
      body = Vec<DepartmentResponse>,
      example = json!(
                {
          "code": 200,
          "message": "Get departments successfully",
          "data": [
            {
              "id": "1w6ajp6l6gooi9g",
              "organizeId": "GFI",
              "managerId": "None",
              "name": "VBI"
            },
            {
              "id": "ojw8a7ibg1ah5gj",
              "organizeId": "string",
              "managerId": "string",
              "name": "string"
            }
          ],
          "error": ""
        }
      )
    ),
  )
)]
pub fn get_departments() -> Router<AppState> {
    async fn get_department_handler(
        State(AppState { department_service, .. }): State<AppState>,
        DepartmentQueryRequest { offset, limit, id, name, organize_id, manager_id }: DepartmentQueryRequest
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
            filters.push(department::pk_department_id::equals(id));
        }

        if let Some(name) = name {
            filters.push(department::name::equals(name));
        }

        if let Some(organize_id) = organize_id {
            filters.push(department::organize_id::equals(organize_id));
        }

        if let Some(manager_id) = manager_id {
            filters.push(department::manager_id::equals(Some(manager_id)));
        }

        let departments: Vec<DepartmentResponse> = department_service
            .get_departments(filters, offset, limit)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();
        Ok(WebResponse::ok("Get departments successfully", departments))
    }
    Router::new().route("/", get(get_department_handler))
}

#[utoipa::path(
  get,
  tag = "Department",
  path = "/api/v1/department/{department_id}",
  params(
    ("department_id" = String, Path, description = "Department ID")
  ),
  responses(
    (
      status = 201,
      description = "Get department by obj id",
      body = DepartmentResponse,
      example = json! (
               {
          "code": 200,
          "message": "Get department by id successfully",
          "data": {
            "id": "1w6ajp6l6gooi9g",
            "organizeId": "GFI",
            "managerId": "None",
            "name": "VBI"
          },
          "error": ""
        } 
      )
    ),
  )
)]
pub fn get_department() -> Router<AppState> {
    async fn get_department_handler(
        State(AppState { department_service, .. }): State<AppState>,
        Path(department_id): Path<String>,
    ) -> WebResult {
        let department: DepartmentResponse = department_service.get_department_by_id(department_id).await?.into();
        Ok(WebResponse::ok("Get department by id successfully", department))
    }
    Router::new().route("/:department_id", get(get_department_handler))
}
