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
        request::{AddMultipleUserToOrg, UpdateUserRequest},
        response::{UserResponse, UserSelectWithPassword},
    },
    WebResult,
};

#[utoipa::path(
  put,
  tag = "User",
  path = "/api/v1/user/{user_id}/add_to_org/{org_id}",
  params(
    ("user_id" = String, Path, description = "User ID"),
    ("org_id" = String, Path, description = "Organize ID")
  ),
  responses(
    (
      status = 200,
      description = "Add user organize successfully",
      body = UserResponse,
      example = json!(
        {
          "code": 200,
          "message": "Add user to organize successfully",
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
pub fn add_to_organize() -> Router<AppState> {
    async fn add_to_organize_handler(
        State(AppState { user_service, .. }): State<AppState>,
        Path((user_id, org_id)): Path<(String, String)>,
        //LoggedInUser(user): LoggedInUser,
    ) -> WebResult {
        let updated_user: UserResponse = user_service
            .add_user_to_organize(user_id, org_id)
            .await?
            .into();
        Ok(WebResponse::ok(
            "Add user to organize successfully",
            updated_user,
        ))
    }
    Router::new().route("/:user_id/add_to_org/:org_id", put(add_to_organize_handler))
}

#[utoipa::path(
  put,
  tag = "User",
  path = "/api/v1/user/add_multiple_to_org",
  request_body(
    content = AddMultipleUserToOrg,
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
pub fn add_multiple_to_org() -> Router<AppState> {
    async fn add_multiple_to_org_handler(
        State(AppState { user_service, .. }): State<AppState>,
        AddMultipleUserToOrg { list_user, org_id }: AddMultipleUserToOrg, //LoggedInUser(user): LoggedInUser,
    ) -> WebResult {
        let mut list_updated: Vec<UserResponse> = vec![];
        for user in list_user {
            let updated_user: UserResponse = user_service
                .add_user_to_organize(user.clone(), org_id.clone())
                .await?
                .into();
            list_updated.push(updated_user)
        }
        Ok(WebResponse::ok(
            "Add user to department successfully",
            list_updated,
        ))
    }
    Router::new().route("/add_multiple_to_org", put(add_multiple_to_org_handler))
}
