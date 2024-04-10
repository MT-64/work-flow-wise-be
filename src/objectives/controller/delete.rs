use crate::users::model::loggedin::LoggedInUser;
use axum::{
    extract::{Path, State},
    routing::delete,
    Router,
};

use crate::{
    helpers::validation::validation_message, response::WebResponse, state::AppState, WebResult,
};

#[utoipa::path(
  delete,
  tag = "Objective",
  path = "/api/v1/objective/delete/{obj_id}",
  params(
    ("obj_id" = String, Path, description = "Objective ID")
  ),
  responses(
    (
      status = 200,
      description = "Deleted objective successfully",
      body = WebResponse,
      example = json!(
        {
          "code": 200,
          "message": "Deleted objective successfully",
          "data": null,
          "error": ""
        }
      )
    )
  )
)]
pub fn delete_obj() -> Router<AppState> {
    async fn delete_obj_handler(
        State(AppState { obj_service, .. }): State<AppState>,
        LoggedInUser(_): LoggedInUser,
        Path(obj_id): Path<String>,
    ) -> WebResult {
        obj_service.delete_obj(obj_id).await?;

        Ok(WebResponse::ok("Deleted objective successfully", ()))
    }
    Router::new().route("/delete/:obj_id", delete(delete_obj_handler))
}
