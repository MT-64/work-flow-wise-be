use axum::{
    extract::{Path, State},
    routing::put,
    Router,
};
use chrono::DateTime;

use crate::{
    helpers::validation::validation_message,
    objectives::model::{request::UpdateObjRequest, response::ObjectiveResponse},
    prisma::objective::{self, deadline, target},
    response::WebResponse,
    state::AppState,
    users::model::loggedin::LoggedInUser,
    WebResult,
};

#[utoipa::path(
  put,
  tag = "Objective",
  path = "/api/v1/objective/update/{obj_id}",
  params(
    ("obj_id" = String, Path, description = "Objective ID")
  ),
  request_body(
    content = UpdateObjRequest,
    description = "Update objective request",
  ),
  responses(
    (
      status = 200,
      description = "Updated objective successfully",
      body = ObjectiveResponse,
      example = json!(
        {
          "code": 200,
          "message": "Updated objective successfully",
          "data": {
                     },
          "error": ""
        }
      )
    )
  )
)]
pub fn update_obj() -> Router<AppState> {
    async fn update_obj_handler(
        State(AppState {
            user_service,
            notification_service,
            obj_service,
            ..
        }): State<AppState>,
        Path(obj_id): Path<String>,
        LoggedInUser(user): LoggedInUser,
        UpdateObjRequest {
            name,
            period_id,
            description,
            target,
            progress,
            deadline,
            achievement,
            expected,
        }: UpdateObjRequest,
    ) -> WebResult {
        let mut changes = vec![];

        if let Some(name) = name {
            changes.push(objective::name::set(name));
        }

        if let Some(description) = description {
            changes.push(objective::description::set(Some(description)));
        }

        if let Some(target) = target {
            changes.push(objective::target::set(target))
        }

        if let Some(achievement) = achievement {
            changes.push(objective::achievement::set(Some(achievement)))
        }
        if let Some(expected) = expected {
            changes.push(objective::expected::set(expected))
        }

        if let Some(period_id) = period_id {
            changes.push(objective::period_id::set(period_id))
        }

        if let Some(deadline) = deadline {
            changes.push(objective::deadline::set(
                DateTime::from_timestamp(deadline, 0)
                    .unwrap()
                    .fixed_offset(),
            ))
        }
        if let Some(progress) = progress {
            changes.push(objective::progress::set(Some(progress)));
        }

        let updated_obj: ObjectiveResponse = obj_service.update_obj(obj_id, changes).await?.into();
        let users = user_service
            .get_users_by_obj(updated_obj.obj_id.clone())
            .await?;
        let message = format!(r#"Mục tiêu {} đã được cập nhật"#, updated_obj.name.clone());
        for user in users.iter() {
            notification_service
                .create_noti(user.pk_user_id.clone(), message.clone(), vec![])
                .await?;
        }

        Ok(WebResponse::ok(
            "Update objective successfully",
            updated_obj,
        ))
    }
    Router::new().route("/update/:obj_id", put(update_obj_handler))
}
