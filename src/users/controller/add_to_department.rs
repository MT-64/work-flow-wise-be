use axum::{
    extract::{Path, State},
    routing::put,
    Router,
};

use crate::{
    helpers::validation::validation_message,
    prisma::user,
    response::WebResponse,
    state::AppState,
    users::model::{
        loggedin::LoggedInUser,
        request::{AddMultipleUserToDepartment, UpdateUserRequest},
        response::{UserResponse, UserSelectWithPassword},
    },
    WebResult,
};

#[utoipa::path(
  put,
  tag = "User",
  path = "/api/v1/user/{user_id}/add_to_department/{department_id}",
  params(
    ("user_id" = String, Path, description = "User ID"),
    ("department_id" = String, Path, description = "Department ID")
  ),
  responses(
    (
      status = 200,
      description = "Add user department successfully",
      body = UserResponse,
      example = json!(
        {
          "code": 200,
          "message": "Add user to department successfully",
          "data": {
            "createdAt": 1696932804946_i64,
            "email": "giang@local.com",
            "firstName": "Azuros",
            "id": "E--_R7geRkFe33WKac5f",
            "image": null,
            "introductionBrief": "Conservative Tech Officer (CTO) @ VSystems Inc.",
            "lastName": "Cloudapi",
            "level": "Beginner",
            "username": "Tester",
            "role": "Subscriber",
            "totalCredit": 0,
            "official": false,
            "updatedAt": 1696933005817_i64
          },
          "error": ""
        }
      )
    )
  )
)]
pub fn add_to_department() -> Router<AppState> {
    async fn add_to_department_handler(
        State(AppState { user_service, .. }): State<AppState>,
        Path((user_id, department_id)): Path<(String, String)>,
        //LoggedInUser(user): LoggedInUser,
    ) -> WebResult {
        let updated_user: UserResponse = user_service
            .add_user_to_department(user_id, department_id)
            .await?
            .into();
        Ok(WebResponse::ok(
            "Add user to department successfully",
            updated_user,
        ))
    }
    Router::new().route(
        "/:user_id/add_to_department/:department_id",
        put(add_to_department_handler),
    )
}

#[utoipa::path(
  put,
  tag = "User",
  path = "/api/v1/user/add_multiple_to_department",
  request_body(
    content = AddMultipleUserToDepartment,
    description = "Create User Request",
  ),

  responses(
    (
      status = 200,
      description = "Add users department successfully",
      body = UserResponse,
      example = json!(
        {
          "code": 200,
          "message": "Add user to department successfully",
          "data": {
            "createdAt": 1696932804946_i64,
            "email": "giang@local.com",
            "firstName": "Azuros",
            "id": "E--_R7geRkFe33WKac5f",
            "image": null,
            "introductionBrief": "Conservative Tech Officer (CTO) @ VSystems Inc.",
            "lastName": "Cloudapi",
            "level": "Beginner",
            "username": "Tester",
            "role": "Subscriber",
            "totalCredit": 0,
            "official": false,
            "updatedAt": 1696933005817_i64
          },
          "error": ""
        }
      )
    )
  )
)]
pub fn add_multiple_to_department() -> Router<AppState> {
    async fn add_multiple_to_department_handler(
        State(AppState { user_service, .. }): State<AppState>,
        AddMultipleUserToDepartment {
            list_user,
            department_id,
        }: AddMultipleUserToDepartment, //LoggedInUser(user): LoggedInUser,
    ) -> WebResult {
        let mut list_updated: Vec<UserResponse> = vec![];
        for user in list_user {
            let updated_user: UserResponse = user_service
                .add_user_to_department(user.clone(), department_id.clone())
                .await?
                .into();
            list_updated.push(updated_user)
        }
        Ok(WebResponse::ok(
            "Add user to department successfully",
            list_updated,
        ))
    }
    Router::new().route(
        "/add_multiple_to_department",
        put(add_multiple_to_department_handler),
    )
}
