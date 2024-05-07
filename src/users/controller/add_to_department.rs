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
        State(AppState {
            department_service,
            notification_service,
            user_service,
            ..
        }): State<AppState>,
        Path((user_id, department_id)): Path<(String, String)>,
        //LoggedInUser(user): LoggedInUser,
    ) -> WebResult {
        let updated_user: UserResponse = user_service
            .add_user_to_department(user_id, department_id.clone())
            .await?
            .into();

        let department = department_service
            .get_department_by_id(department_id.clone())
            .await?;

        let message = format!(
            r#"You have assign to department {}"#,
            department.name.clone()
        );

        notification_service
            .create_noti(updated_user.id.clone(), message.clone(), vec![])
            .await?;

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
        State(AppState {
            user_service,
            notification_service,
            department_service,
            ..
        }): State<AppState>,
        AddMultipleUserToDepartment {
            list_user,
            department_id,
        }: AddMultipleUserToDepartment, //LoggedInUser(user): LoggedInUser,
    ) -> WebResult {
        let department = department_service
            .get_department_by_id(department_id.clone())
            .await?;

        let message = format!(
            r#"You have assign to department {}"#,
            department.name.clone()
        );

        let mut list_updated: Vec<UserResponse> = vec![];
        for user in list_user {
            let updated_user: UserResponse = user_service
                .add_user_to_department(user.clone(), department_id.clone())
                .await?
                .into();

            notification_service
                .create_noti(updated_user.id.clone(), message.clone(), vec![])
                .await?;

            list_updated.push(updated_user);
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

#[utoipa::path(
  put,
  tag = "User",
  path = "/api/v1/user/remove_user_department/{user_id}",
  params(
    ("user_id" = String, Path, description = "User ID")
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
pub fn remove_user_department() -> Router<AppState> {
    async fn remove_user_department_handler(
        State(AppState {
            user_service,
            department_service,
            notification_service,
            ..
        }): State<AppState>,
        Path(user_id): Path<String>,
    ) -> WebResult {
        let user = user_service.get_user_by_id(user_id.clone()).await?;

        let department = department_service
            .get_department_by_id(user.department_id.expect("User not in department").clone())
            .await?;

        let message = format!(
            r#"You have removed from department {}"#,
            department.name.clone()
        );

        let updated_user: UserResponse = user_service.remove_user_department(user_id).await?.into();

        notification_service
            .create_noti(updated_user.id.clone(), message.clone(), vec![])
            .await?;

        Ok(WebResponse::ok(
            "Remove user department successfully",
            updated_user,
        ))
    }
    Router::new().route(
        "/remove_user_department/:user_id",
        put(remove_user_department_handler),
    )
}
