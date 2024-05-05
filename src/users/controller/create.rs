use axum::{extract::State, routing::post, Router};

use crate::{
    prisma::user,
    response::WebResponse,
    state::AppState,
    users::model::{
        loggedin::LoggedInAdmin,
        request::CreateUserRequest,
        response::{UserResponse, UserSelect},
    },
    WebResult,
};

#[utoipa::path(
  post,
  tag = "User",
  path = "/api/v1/user/create",
  request_body(
    content = CreateUserRequest,
    description = "Create User Request",
  ),
  responses(
    (
      status = 201,
      description = "User created",
      body = UserResponse,
      example = json! (
        {
          "code": 201,
          "message": "Created new user successfully",
          "data": {
            "createdAt": 1696932804946_i64,
            "email": "tester@local.com",
            "firstName": null,
            "id": "E--_R7geRkFe33WKac5f",
            "image": null,
            "introductionBrief": null,
            "lastName": null,
            "level": "Beginner",
            "username": "Tester",
            "role": "Subscriber",
            "totalCredit": 0,
            "official": false,
            "updatedAt": 1696932804946_i64
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn create_user() -> Router<AppState> {
    async fn create_user_handler(
        State(AppState {
            folder_service,
            user_service,
            ..
        }): State<AppState>,
        CreateUserRequest {
            username,
            email,
            password,
            first_name,
            last_name,
            gender,
            ..
        }: CreateUserRequest,
    ) -> WebResult {
        let mut params = vec![];

        params.push(user::first_name::set(first_name));
        params.push(user::last_name::set(last_name));
        params.push(user::gender::set(gender));

        let new_user: UserResponse = user_service
            .create_user(username, email.unwrap_or_default(), password, params)
            .await?
            .into();

        let folder = folder_service
            .create_root_folder(new_user.id.clone())
            .await?;

        Ok(WebResponse::created(
            "Created user successfully",
            UserResponse::from(new_user),
        ))
    }
    Router::new().route("/create", post(create_user_handler))
}

#[utoipa::path(
  post,
  tag = "User",
  path = "/api/v1/user/admin_create",
  request_body(
    content = CreateUserRequest,
    description = "Create User Request",
  ),
  responses(
    (
      status = 201,
      description = "User created",
      body = UserResponse,
      example = json! (
        {
          "code": 201,
          "message": "Created new user successfully",
          "data": {
            "createdAt": 1696932804946_i64,
            "email": "tester@local.com",
            "firstName": null,
            "id": "E--_R7geRkFe33WKac5f",
            "image": null,
            "introductionBrief": null,
            "lastName": null,
            "level": "Beginner",
            "username": "Tester",
            "role": "Subscriber",
            "totalCredit": 0,
            "official": false,
            "updatedAt": 1696932804946_i64
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn admin_create_user() -> Router<AppState> {
    async fn admin_create_user_handler(
        State(AppState {
            folder_service,
            user_service,
            ..
        }): State<AppState>,
        LoggedInAdmin(_): LoggedInAdmin,
        CreateUserRequest {
            username,
            email,
            password,
            ..
        }: CreateUserRequest,
    ) -> WebResult {
        let new_user: UserResponse = user_service
            .create_user(username, email.unwrap_or_default(), password, vec![])
            .await?
            .into();

        let folder = folder_service
            .create_root_folder(new_user.id.clone())
            .await?;

        Ok(WebResponse::created(
            "Created user successfully",
            UserResponse::from(new_user),
        ))
    }
    Router::new().route("/admin_create", post(admin_create_user_handler))
}
