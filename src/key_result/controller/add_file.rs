use axum::{
    extract::{Path, State},
    routing::put,
    Router,
};
use chrono::DateTime;
use prisma_client_rust::PrismaValue;

use crate::{
    error::ErrorResponse,
    helpers::validation::validation_message,
    key_result::model::{
        request::{AddFileRequest, GradingKr},
        response::{FileSharedResponse, KeyResultResponse},
    },
    prisma::{
        key_result::{self, deadline},
        objective,
    },
    response::WebResponse,
    state::AppState,
    users::model::loggedin::LoggedInUser,
    WebResult,
};

#[utoipa::path(
  put,
  tag = "Key Result",
  path = "/api/v1/kr/add_file/{kr_id}",
  params(
    ("kr_id" = String, Path, description = "Keyresult ID")
  ),

  request_body(
    content = AddFileRequest,
    description = "Update keyresult request",
  ),
  responses(
    (
      status = 200,
      description = "Updated keyresult successfully",
      body = KeyResultResponse,
      example = json!(
        {
          "code": 200,
          "message": "Updated keyresult successfully",
          "data": {
                     },
          "error": ""
        }
      )
    )
  )
)]
pub fn add_file() -> Router<AppState> {
    async fn add_file_handler(
        State(AppState {
            keyresult_service, ..
        }): State<AppState>,
        Path(kr_id): Path<String>,
        LoggedInUser(user): LoggedInUser,
        AddFileRequest {
            file_path,
            virutal_path,
        }: AddFileRequest,
    ) -> WebResult {
        let mut changes = vec![];

        let updated_kr: FileSharedResponse = keyresult_service
            .add_file_to_kr(kr_id, file_path, virutal_path, changes)
            .await?
            .into();

        Ok(WebResponse::ok("Add file to kr successfully", updated_kr))
    }
    Router::new().route("/add_file/:kr_id", put(add_file_handler))
}
