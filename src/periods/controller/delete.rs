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
  tag = "Period",
  path = "/api/v1/period/delete/{period_id}",
  params(
    ("period_id" = String, Path, description = "Period ID")
  ),
  responses(
    (
      status = 200,
      description = "Deleted period successfully",
      body = WebResponse,
      example = json!(
        {
          "code": 200,
          "message": "Deleted period successfully",
          "data": null,
          "error": ""
        }
      )
    )
  )
)]
pub fn delete_period() -> Router<AppState> {
    async fn delete_period_handler(
        State(AppState { period_service, .. }): State<AppState>,
        LoggedInUser(_): LoggedInUser,
        Path(period_id): Path<String>,
    ) -> WebResult {
        period_service.delete_period(period_id).await?;

        Ok(WebResponse::ok("Deleted period successfully", ()))
    }
    Router::new().route("/delete/:period_id", delete(delete_period_handler))
}
