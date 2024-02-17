use axum::{
    extract::{Path, State},
    routing::put,
    Router,
};
use chrono::DateTime;

use crate::{
    department::model::{request::UpdateDepartmentRequest, response::DepartmentResponse},
    helpers::validation::validation_message,
    response::WebResponse,
    state::AppState,
    users::model::loggedin::LoggedInUser,
    WebResult,
};

use crate::prisma::department;

#[utoipa::path(
  put,
  tag = "Department",
  path = "/api/v1/department/update/{department_id}",
  params(
    ("department_id" = String, Path, description = "Department ID")
  ),

  request_body(
    content = UpdateDepartmentRequest,
  content_type = "multipart/form-data",
    description = "Update department request",
  ),
  responses(
    (
      status = 200,
      description = "Updated department successfully",
      body = DepartmentResponse,
      example = json!(
        {
          "code": 200,
          "message": "Updated department successfully",
          "data": {
                     },
          "error": ""
        }
      )
    )
  )
)]
pub fn update_department() -> Router<AppState> {
    async fn update_department_handler(
        State(AppState {
            department_service, ..
        }): State<AppState>,
        Path(department_id): Path<String>,
        LoggedInUser(user): LoggedInUser,
        UpdateDepartmentRequest {
            manager_id,
            organize_id,
            name,
        }: UpdateDepartmentRequest,
    ) -> WebResult {
        let mut changes = vec![];

        if let Some(name) = name {
            changes.push(department::name::set(name));
        }

        changes.push(department::manager_id::set(manager_id));

        if let Some(organize_id) = organize_id {
            changes.push(department::organize_id::set(organize_id))
        }

        let updated_department: DepartmentResponse = department_service
            .update_department(department_id, changes)
            .await?
            .into();
        Ok(WebResponse::ok(
            "Update department successfully",
            updated_department,
        ))
    }
    Router::new().route("/update/:department_id", put(update_department_handler))
}
