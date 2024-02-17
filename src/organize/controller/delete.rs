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
  tag = "Organize",
  path = "/api/v1/organize/delete/{org_id}",
  params(
    ("org_id" = String, Path, description = "Organize ID")
  ),
  responses(
    (
      status = 200,
      description = "Deleted organize successfully",
      body = WebResponse,
      example = json!(
        {
          "code": 200,
          "message": "Deleted organize successfully",
          "data": null,
          "error": ""
        }
      )
    )
  )
)]
pub fn delete_organize() -> Router<AppState> {
    async fn delete_organize_handler(
        State(AppState {
            organize_service, ..
        }): State<AppState>,
        LoggedInUser(_): LoggedInUser,
        Path(org_id): Path<String>,
    ) -> WebResult {
        organize_service.delete_organize(org_id).await?;

        Ok(WebResponse::ok("Deleted organize successfully", ()))
    }
    Router::new().route("/delete/:org_id", delete(delete_organize_handler))
}
