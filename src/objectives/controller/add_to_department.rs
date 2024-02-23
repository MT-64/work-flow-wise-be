use axum::{
    extract::{Path, State},
    routing::post,
    Router,
};

use crate::{
    objectives::model::{request::CreateObjRequest, response::ObjectiveResponse},
    prisma::{self, objective},
    response::WebResponse,
    state::AppState,
    WebResult,
};

#[utoipa::path(
  post,
  tag = "Objective",
  path = "/api/v1/objective/{obj_id}/add_to_department/{department_id}",
  params(
    ("obj_id" = String, Path, description = "Objective ID"),
    ("department_id" = String, Path, description = "Department ID")
  ),
  responses(
    (
      status = 201,
      description = "Add objective to department successfully",
      body = WebResponse,
      example = json! (
        {
          "code": 201,
          "message": "Add objective to department successfully",
          "data": null,
          "error": ""
        }
      )
    ),
  )
)]
pub fn add_to_department() -> Router<AppState> {
    async fn add_to_department_handler(
        State(AppState { obj_service, .. }): State<AppState>,
        Path((obj_id, department_id)): Path<(String, String)>,
    ) -> WebResult {
        obj_service.add_to_department(obj_id, department_id).await?;

        Ok(WebResponse::created(
            "Add objective to department sucessfully",
            (),
        ))
    }
    Router::new().route(
        "/:obj_id/add_to_department/:department_id",
        post(add_to_department_handler),
    )
}
