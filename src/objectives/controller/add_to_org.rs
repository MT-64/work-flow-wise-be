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
  path = "/api/v1/objective/{obj_id}/add_to_organize/{org_id}",
  params(
    ("obj_id" = String, Path, description = "Objective ID"),
    ("org_id" = String, Path, description = "Org ID")
  ),
  responses(
    (
      status = 201,
      description = "Add objective to organize successfully",
      body = WebResponse,
      example = json! (
        {
          "code": 201,
          "message": "Add objective to organize successfully",
          "data": null,
          "error": ""
        }
      )
    ),
  )
)]
pub fn add_to_organize() -> Router<AppState> {
    async fn add_to_organize_handler(
        State(AppState { obj_service, .. }): State<AppState>,
        Path((obj_id, org_id)): Path<(String, String)>,
    ) -> WebResult {
        obj_service.add_to_org(obj_id, org_id).await?;

        Ok(WebResponse::created(
            "Add objective to organize sucessfully",
            (),
        ))
    }
    Router::new().route(
        "/:obj_id/add_to_organize/:org_id",
        post(add_to_organize_handler),
    )
}
