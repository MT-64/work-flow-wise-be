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
        request::{GradingKr, UpdateKrRequest},
        response::KeyResultResponse,
    },
    prisma::key_result::{self, deadline},
    response::WebResponse,
    state::AppState,
    users::model::loggedin::LoggedInUser,
    WebResult,
};

#[utoipa::path(
  put,
  tag = "Key Result",
  path = "/api/v1/kr/update/{kr_id}",
  params(
    ("kr_id" = String, Path, description = "Keyresult ID")
  ),

  request_body(
    content = UpdateKrRequest,
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
pub fn update_kr() -> Router<AppState> {
    async fn update_kr_handler(
        State(AppState {
            keyresult_service, ..
        }): State<AppState>,
        Path(kr_id): Path<String>,
        LoggedInUser(user): LoggedInUser,
        UpdateKrRequest {
            name,
            user_id,
            objective_id,
            description,
            target,
            progress,
            deadline,
        }: UpdateKrRequest,
    ) -> WebResult {
        let mut changes = vec![];

        if let Some(name) = name {
            changes.push(key_result::name::set(name));
        }

        if let Some(description) = description {
            changes.push(key_result::description::set(description));
        }

        if let Some(target) = target {
            changes.push(key_result::target::set(target))
        }

        if let Some(progress) = progress {
            changes.push(key_result::progress::set(progress));
        }

        if let Some(deadline) = deadline {
            changes.push(key_result::deadline::set(
                DateTime::from_timestamp(deadline, 0)
                    .unwrap()
                    .fixed_offset(),
            ))
        }

        let updated_kr: KeyResultResponse =
            keyresult_service.update_kr(kr_id, changes).await?.into();
        Ok(WebResponse::ok("Update keyresult successfully", updated_kr))
    }
    Router::new().route("/update/:kr_id", put(update_kr_handler))
}

#[utoipa::path(
  put,
  tag = "Key Result",
  path = "/api/v1/kr/grading_kr/{kr_id}",
  params(
    ("kr_id" = String, Path, description = "Keyresult ID")
  ),

  request_body(
    content = GradingKr,
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
pub fn grading_kr() -> Router<AppState> {
    async fn grading_kr_handler(
        State(AppState {
            obj_service,
            keyresult_service,
            ..
        }): State<AppState>,
        Path(kr_id): Path<String>,
        LoggedInUser(user): LoggedInUser,
        GradingKr { grade }: GradingKr,
    ) -> WebResult {
        let mut changes = vec![];

        let kr = keyresult_service.get_kr_by_id(kr_id.clone()).await?;

        let obj = obj_service.get_obj_by_id(kr.objective_id).await?;

        if obj.supervisor_id != user.pk_user_id {
            return Err(ErrorResponse::Permissions);
        }

        changes.push(key_result::supervior_grade::set(grade));
        changes.push(key_result::status::set(true));

        let updated_kr: KeyResultResponse =
            keyresult_service.update_kr(kr_id, changes).await?.into();
        Ok(WebResponse::ok("Update keyresult successfully", updated_kr))
    }
    Router::new().route("/grading_kr/:kr_id", put(grading_kr_handler))
}
