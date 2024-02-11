use axum::{extract::State, routing::get, Router};

use crate::{
    response::WebResponse,
    state::AppState,
    users::model::{
        loggedin::LoggedInUser,
        response::{UserResponse, UserSelect},
    },
    WebResult,
};

#[utoipa::path(
  get,
  tag = "User",
  path = "/api/v1/user/check_profile",
  responses(
    (
      status = 200,
      description = "Get user profile successfully",
      body = UserResponse,
      example = json! (
        {
          "code": 200,
          "message": "Get user profile successfully",
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
    )
  )
)]
pub fn profile() -> Router<AppState> {
    async fn profile_handler(
        State(AppState { user_service, .. }): State<AppState>,
        LoggedInUser(UserSelect {
            pk_user_id: user_id,
            ..
        }): LoggedInUser,
    ) -> WebResult {
        let filtered_user: UserResponse = user_service.get_user_by_id(user_id).await?.into();
        Ok(WebResponse::ok("Get user's profile success", filtered_user))
    }
    Router::new().route("/profile", get(profile_handler))
}
