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
  tag = "Key Result",
  path = "/api/v1/kr/delete/{kr_id}",
  params(
    ("kr_id" = String, Path, description = "Keyresult ID")
  ),
  responses(
    (
      status = 200,
      description = "Deleted keyresult successfully",
      body = WebResponse,
      example = json!(
        {
          "code": 200,
          "message": "Deleted keyresult successfully",
          "data": null,
          "error": ""
        }
      )
    )
  )
)]
pub fn delete_kr() -> Router<AppState> {
    async fn delete_kr_handler(
        State(AppState {
            keyresult_service, ..
        }): State<AppState>,
        LoggedInUser(_): LoggedInUser,
        Path(kr_id): Path<String>,
    ) -> WebResult {
        keyresult_service.delete_kr(kr_id).await?;

        Ok(WebResponse::ok("Deleted keyresult successfully", ()))
    }
    Router::new().route("/delete/:kr_id", delete(delete_kr_handler))
}
