use crate::prisma::{self, objective_on_department::department_id, user};
use axum::{extract::Path, extract::State, routing::get, Router};
use chrono::DateTime;
use prisma_client_rust::query_core::schema_builder::constants::filters;

use crate::{
    response::WebResponse,
    state::AppState,
    users::model::{loggedin::LoggedInUser, request::UserQueryRequest, response::UserResponse},
    WebResult,
};

#[utoipa::path(
  get,
  tag = "User",
  path = "/api/v1/user",
  params(
    ("offset" = inline(Option<i64>), Query, description = "Starting point"),
    ("limit" = inline(Option<i32>), Query, description = "Limit"),
    ("id" = inline(Option<String>), Query, description = "User id"),
    ("departmentId" = inline(Option<String>), Query, description = "department_id"),
    ("organizeId" = inline(Option<String>), Query, description = "organize_id"),
    ("firstName" = inline(Option<String>), Query, description = "User first name"),
    ("lastName" = inline(Option<String>), Query, description = "User last name"),
    ("nickname" = inline(Option<String>), Query, description = "User nickname"),
    ("email" = inline(Option<String>), Query, description = "User email"),
    ("role" = inline(Option<String>), Query, description = "User role"),
    ("createdAt" = inline(Option<i64>), Query, description = "User created at"),
    ("updatedAt" = inline(Option<i64>), Query, description = "User updated at"),
  ),
  responses(
    (
      status = 200,
      description = "Get users",
      body = Vec<UserResponse>,
      example = json!(
        {
          "code": 200,
          "message": "Get all users successfully",
          "data": [
            {
              "createdAt": 1696883872894_i64,
              "email": "a@a.com",
              "firstName": "John",
              "id": "kfqfiyd3veyCMh0s42Cr",
              "image": "john-avatar.png",
              "introductionBrief": "My name is John Doe",
              "lastName": "Doe",
              "level": "Beginner",
              "nickname": "john",
              "role": "Subscriber",
              "totalCredit": 10,
              "updatedAt": 1696883872894_i64
            },
            {
              "createdAt": 1696883872895_i64,
              "email": "b@b.com",
              "firstName": "Jane",
              "id": "0p3LRCqfkes4cI1UrSmj",
              "image": "jane-avatar.png",
              "introductionBrief": "My name is Jane Doe",
              "lastName": "Doe",
              "level": "Senior",
              "nickname": "jane",
              "role": "Subscriber",
              "totalCredit": 30,
              "updatedAt": 1696883872895_i64
            },
          ],
          "error": ""
        }
      )
    ),
  )
)]
pub fn get_users() -> Router<AppState> {
    async fn get_users_handler(
        State(AppState { user_service, .. }): State<AppState>,
        UserQueryRequest {
            offset,
            limit,
            id,
            department_id,
            organize_id,
            first_name,
            last_name,
            username,
            email,
            role,
            created_at,
            updated_at,
        }: UserQueryRequest,
    ) -> WebResult {
        let offset = offset.unwrap_or(0);

        let limit = match limit {
            Some(limit) => match limit {
                0..=50 => limit,
                _ => 10,
            },
            None => 10,
        };

        let mut filters = vec![];

        if let Some(id) = id {
            filters.push(user::pk_user_id::equals(id));
        }

        if let Some(department_id) = department_id {
            filters.push(user::department_id::equals(Some(department_id)));
        }
        if let Some(organize_id) = organize_id {
            filters.push(user::organize_id::equals(Some(organize_id)));
        }

        if let Some(first_name) = first_name {
            filters.push(user::first_name::equals(Some(first_name)));
        }

        if let Some(last_name) = last_name {
            filters.push(user::last_name::equals(Some(last_name)));
        }

        if let Some(username) = username {
            filters.push(user::username::equals(username));
        }
        if let Some(email) = email {
            filters.push(user::email::equals(email));
        }
        if let Some(role) = role {
            let p_role = match role.trim() {
                "Admin" => prisma::Role::Admin,
                "Owner" => prisma::Role::Owner,
                "Manager" => prisma::Role::Manager,
                _ => prisma::Role::Employee,
            };

            filters.push(user::role::equals(p_role));
        }
        if let Some(created_at) = created_at {
            filters.push(user::created_at::gte(
                DateTime::from_timestamp(created_at, 0)
                    .unwrap()
                    .fixed_offset(),
            ));
        }
        if let Some(updated_at) = updated_at {
            filters.push(user::updated_at::gte(
                DateTime::from_timestamp(updated_at, 0)
                    .unwrap()
                    .fixed_offset(),
            ));
        }

        let users: Vec<UserResponse> = user_service
            .get_users(filters, offset, limit)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();

        Ok(WebResponse::ok("Get users successfully", users))
    }
    Router::new().route("/", get(get_users_handler))
}

#[utoipa::path(
  get,
  tag = "User",
  path = "/api/v1/user/{user_id}",
  params(
    ("user_id" = String, Path, description = "User ID")
  ),
  responses(
    (
      status = 201,
      description = "Get user by user id",
      body = UserResponse,
      example = json! (
        {
          "code": 200,
          "message": "Get user by user id successfully",
          "data": {
            "createdAt": 1700299087279_i64,
            "email": "d@d.com",
            "firstName": "Jim",
            "id": "EIWFI7wzXD5-PPFlPVVS",
            "image": "fedora:8000/user/avatar/EIWFI7wzXD5-PPFlPVVS",
            "introductionBrief": "I am Jimmy Johnson",
            "lastName": "Johnson",
            "level": "Beginner",
            "nickname": "jim",
            "paginationId": 4,
            "role": "Subscriber",
            "totalCredit": 20,
            "updatedAt": 1700299087279_i64
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn get_user() -> Router<AppState> {
    async fn get_user_handler(
        State(AppState { user_service, .. }): State<AppState>,
        Path(user_id): Path<String>,
    ) -> WebResult {
        let user: UserResponse = user_service.get_user_by_id(user_id).await?.into();
        Ok(WebResponse::ok("Get user by id successfully", user))
    }
    Router::new().route("/:user_id", get(get_user_handler))
}

#[utoipa::path(
  get,
  tag = "User",
  path = "/api/v1/user/get_by_obj/{obj_id}",
  params(
    ("obj_id" = String, Path, description = "objective ID")
  ),
  responses(
    (
      status = 201,
      description = "Get user by obj id",
      body = ObjectiveResponse,
      example = json! (
        {
          "code": 200,
          "message": "Get user by obj id successfully",
          "data": {
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn get_users_by_obj() -> Router<AppState> {
    async fn get_users_by_obj_handler(
        State(AppState { user_service, .. }): State<AppState>,
        Path(obj_id): Path<String>,
    ) -> WebResult {
        let users: Vec<UserResponse> = user_service
            .get_users_by_obj(obj_id)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();
        Ok(WebResponse::ok(
            "Get users by objective id successfully",
            users,
        ))
    }
    Router::new().route("/get_by_obj/:obj_id", get(get_users_by_obj_handler))
}

#[utoipa::path(
  get,
  tag = "User",
  path = "/api/v1/user/get_info_by_jwt",
  responses(
    (
      status = 201,
      description = "Get user by jwt",
      body = ObjectiveResponse,
      example = json! (
        {
          "code": 200,
          "message": "Get user by jwt successfully",
          "data": {
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn get_user_by_jwt() -> Router<AppState> {
    async fn get_users_by_jwt_handler(
        State(AppState { user_service, .. }): State<AppState>,
        LoggedInUser(user): LoggedInUser,
    ) -> WebResult {
        let user: UserResponse = user_service.get_user_by_id(user.pk_user_id).await?.into();
        Ok(WebResponse::ok("Get users by jwt successfully", user))
    }
    Router::new().route("/get_info_by_jwt", get(get_users_by_jwt_handler))
}
