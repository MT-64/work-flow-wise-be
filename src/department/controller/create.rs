use axum::{extract::State, routing::post, Router};

use crate::{
    department::model::{request::CreateDepartmentRequest, response::DepartmentResponse},
    prisma::{self, department},
    response::WebResponse,
    state::AppState,
    WebResult,
};

#[utoipa::path(
  post,
  tag = "Department",
  path = "/api/v1/department/create",
  request_body(
    content = CreateDepartmentRequest,
    description = "Create Objective Request",
  ),
  responses(
    (
      status = 201,
      description = "Department created",
      body = DepartmentResponse,
      example = json! (
        {
          "code": 201,
          "message": "Created department sucessfully",
          "data": {
            "id": "1w6ajp6l6gooi9g",
            "organizeId": "GFI",
            "managerId": "None",
            "name": "VBI"
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn create_department() -> Router<AppState> {
    async fn create_department_handler(
        State(AppState {
            department_service, ..
        }): State<AppState>,
        CreateDepartmentRequest {
            manager_id,
            organize_id,
            name,
        }: CreateDepartmentRequest,
    ) -> WebResult {
        let mut params = vec![];

        params.push(department::manager_id::set(manager_id));

        let new_department: DepartmentResponse = department_service
            .create_department(organize_id, name, params)
            .await?
            .into();

        Ok(WebResponse::created(
            "Created department sucessfully",
            DepartmentResponse::from(new_department),
        ))
    }
    Router::new().route("/create", post(create_department_handler))
}
