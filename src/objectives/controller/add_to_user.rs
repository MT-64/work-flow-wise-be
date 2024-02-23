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
  path = "/api/v1/objective/{obj_id}/add_to_user/{user_id}",
  params(
    ("obj_id" = String, Path, description = "Objective ID"),
    ("user_id" = String, Path, description = "User ID")
  ),
  responses(
    (
      status = 201,
      description = "Add objective to user successfully",
      body = WebResponse,
      example = json! (
        {
          "code": 201,
          "message": "Add objective to user successfully",
          "data": null,
          "error": ""
        }
      )
    ),
  )
)]
pub fn add_to_user() -> Router<AppState> {
    async fn add_to_user_handler(
        State(AppState { obj_service, .. }): State<AppState>,
        Path((obj_id, user_id)): Path<(String, String)>,
    ) -> WebResult {
        obj_service.add_to_user(obj_id, user_id).await?;

        Ok(WebResponse::created(
            "Add objective to user sucessfully",
            (),
        ))
    }
    Router::new().route("/:obj_id/add_to_user/:user_id", post(add_to_user_handler))
}
