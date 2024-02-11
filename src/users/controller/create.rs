use axum::{extract::State, routing::post, Router};

use crate::{
    response::WebResponse,
    state::AppState,
    users::model::{
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
        State(AppState { user_service, .. }): State<AppState>,
        CreateUserRequest {
            username,
            email,
            password,
            ..
        }: CreateUserRequest,
    ) -> WebResult {
        let new_user: UserResponse = user_service
            .create_user(username, email.unwrap_or_default(), password)
            .await?
            .into();

        // folder_service
        //     .create_root_folder(new_user.pk_user_id.clone())
        //     .await?;

        Ok(WebResponse::created(
            "Created user successfully",
            UserResponse::from(new_user),
        ))
    }
    Router::new().route("/create", post(create_user_handler))
}
