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
  tag = "Department",
  path = "/api/v1/depeartment/delete/{department_id}",
  params(
    ("department_id" = String, Path, description = "Department ID")
  ),
  responses(
    (
      status = 200,
      description = "Deleted department successfully",
      body = WebResponse,
      example = json!(
        {
          "code": 200,
          "message": "Deleted department successfully",
          "data": null,
          "error": ""
        }
      )
    )
  )
)]
pub fn delete_department() -> Router<AppState> {
    async fn delete_department_handler(
        State(AppState {
            department_service, ..
        }): State<AppState>,
        LoggedInUser(_): LoggedInUser,
        Path(department_id): Path<String>,
    ) -> WebResult {
        department_service.delete_department(department_id).await?;

        Ok(WebResponse::ok("Deleted objective successfully", ()))
    }
    Router::new().route("/delete/:department_id", delete(delete_department_handler))
}
