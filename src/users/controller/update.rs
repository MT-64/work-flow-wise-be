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
        loggedin::{LoggedInAdmin, LoggedInUser},
        request::{UpdateRoleRequest, UpdateUserRequest},
        response::{UserResponse, UserSelectWithPassword},
    },
    WebResult,
};

#[utoipa::path(
  put,
  tag = "User",
  path = "/api/v1/user/update",
  request_body(
    content = UpdateUserRequest,
    description = "Update user request",
  ),
  responses(
    (
      status = 200,
      description = "Updated user successfully",
      body = UserResponse,
      example = json!(
        {
          "code": 200,
          "message": "Updated user successfully",
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
pub fn update_user() -> Router<AppState> {
    async fn update_user_handler(
        State(AppState { user_service, .. }): State<AppState>,
        LoggedInUser(user): LoggedInUser,
        UpdateUserRequest {
            username,
            email,
            password,
            new_password,
            role,
            first_name,
            last_name,
            gender,
            introduction_brief,
            image,
            ..
        }: UpdateUserRequest,
    ) -> WebResult {
        let mut changes = vec![];

        if let Some(username) = username {
            changes.push(user::username::set(username))
        }
        if let Some(email) = email {
            changes.push(user::email::set(email))
        }
        if let Some(new_password) = new_password {
            changes.push(user::password::set(new_password))
        }

        if let Some(new_role) = role {
            changes.push(user::role::set(new_role))
        }
        if let Some(first_name) = first_name {
            changes.push(user::first_name::set(Some(first_name)))
        }
        if let Some(last_name) = last_name {
            changes.push(user::last_name::set(Some(last_name)))
        }
        if let Some(gender) = gender {
            changes.push(user::gender::set(Some(gender)))
        }
        if let Some(introduction_brief) = introduction_brief {
            changes.push(user::introduction_brief::set(Some(introduction_brief)))
        }
        if let Some(image) = image {
            changes.push(user::image::set(Some(image)))
        }

        let updated_user: UserResponse = user_service
            .update_user(user.pk_user_id, changes)
            .await?
            .into();
        Ok(WebResponse::ok("Update user successfully", updated_user))
    }
    Router::new().route("/update", put(update_user_handler))
}

#[utoipa::path(
  put,
  tag = "User",
  path = "/api/v1/user/update_role/{user_id}",
  params(
     ("user_id" = String, Path, description = "User ID")
   ),
  request_body(
    content = UpdateRoleRequest,
    description = "Update user role request",
  ),
  responses(
    (
      status = 200,
      description = "Updated user successfully",
      body = UserResponse,
      example = json!(
        {
          "code": 200,
          "message": "Updated user successfully",
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
pub fn update_user_role() -> Router<AppState> {
    async fn update_user_role_handler(
        State(AppState { user_service, .. }): State<AppState>,
        Path(user_id): Path<String>,
        LoggedInAdmin(user): LoggedInAdmin,
        UpdateRoleRequest { role }: UpdateRoleRequest,
    ) -> WebResult {
        let mut changes = vec![];

        if let Some(new_role) = role {
            changes.push(user::role::set(new_role))
        }

        let updated_user: UserResponse = user_service.update_user(user_id, changes).await?.into();
        Ok(WebResponse::ok("Update user successfully", updated_user))
    }
    Router::new().route("/update_role/:user_id", put(update_user_role_handler))
}
