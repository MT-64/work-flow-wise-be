use crate::users::model::loggedin::LoggedInUser;
use axum::{extract::State, routing::delete, Router};

use crate::{
    helpers::validation::validation_message,
    response::WebResponse,
    state::AppState,
    users::model::{request::DeleteUserRequest, response::UserSelectWithPassword},
    WebResult,
};

#[utoipa::path(
  delete,
  tag = "User",
  path = "/api/v1/user/delete",
  request_body(
    content = DeleteUserRequest,
    description = "Delete User Request",
  ),
  responses(
    (
      status = 200,
      description = "Deleted user successfully",
      body = WebResponse,
      example = json!(
        {
          "code": 200,
          "message": "Deleted user successfully",
          "data": null,
          "error": ""
        }
      )
    )
  )
)]
pub fn delete_user() -> Router<AppState> {
    async fn delete_user_handler(
        State(AppState { user_service, .. }): State<AppState>,
        LoggedInUser(user): LoggedInUser,
        DeleteUserRequest { password, .. }: DeleteUserRequest,
    ) -> WebResult {
        user_service.delete_user(user.pk_user_id).await?;

        Ok(WebResponse::ok("Deleted user successfully", ()))
    }
    Router::new().route("/delete", delete(delete_user_handler))
}
