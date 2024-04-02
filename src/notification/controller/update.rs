use axum::{
    extract::{Path, State},
    routing::put,
    Router,
};
use chrono::DateTime;

use crate::{
    helpers::validation::validation_message,
    notification::model::response::NotificationResponse,
    prisma::notification::{self},
    response::WebResponse,
    state::AppState,
    users::model::loggedin::LoggedInUser,
    WebResult,
};

#[utoipa::path(
  put,
  tag = "Notification",
  path = "/api/v1/notification/update_status/{notification_id}",
  params(
    ("kr_id" = String, Path, description = "Keyresult ID")
  ),

  responses(
    (
      status = 200,
      description = "Updated status notification successfully",
      body = NotificationResponse,
      example = json!(
        {
          "code": 200,
          "message": "Updated status notification successfully",
          "data": {
                     },
          "error": ""
        }
      )
    )
  )
)]
pub fn update_noti() -> Router<AppState> {
    async fn update_noti_handler(
        State(AppState {
            notification_service,
            ..
        }): State<AppState>,
        Path(notification_id): Path<String>,
        LoggedInUser(user): LoggedInUser,
    ) -> WebResult {
        let mut changes = vec![];

        changes.push(notification::status::set(true));

        let updated_noti: NotificationResponse = notification_service
            .update_noti(notification_id, changes)
            .await?
            .into();
        Ok(WebResponse::ok(
            "Update status notification successfully",
            updated_noti,
        ))
    }
    Router::new().route("/update_status/:notification_id", put(update_noti_handler))
}
