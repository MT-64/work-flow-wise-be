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
  path = "/api/v1/objective/update/{objective_id}",
  params(
    ("obj_id" = String, Path, description = "Objective ID")
  ),

  request_body(
    content = UpdateObjRequest,
  content_type = "multipart/form-data",
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
        State(AppState { obj_service, .. }): State<AppState>,
        Path(obj_id): Path<String>,
        LoggedInUser(user): LoggedInUser,
        UpdateObjRequest {
            name,
            description,
            target,
            progress,
            deadline,
        }: UpdateObjRequest,
    ) -> WebResult {
        let mut changes = vec![];

        if let Some(name) = name {
            changes.push(objective::name::set(name));
        }

        changes.push(objective::description::set(description));

        if let Some(target) = target {
            changes.push(objective::target::set(target))
        }

        if let Some(deadline) = deadline {
            changes.push(objective::deadline::set(
                DateTime::from_timestamp(deadline, 0)
                    .unwrap()
                    .fixed_offset(),
            ))
        }

        changes.push(objective::progress::set(progress));

        let updated_obj: ObjectiveResponse = obj_service.update_obj(obj_id, changes).await?.into();
        Ok(WebResponse::ok(
            "Update objective successfully",
            updated_obj,
        ))
    }
    Router::new().route("/update/:obj_id", put(update_obj_handler))
}
