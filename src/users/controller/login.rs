use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, State},
    http::{HeaderName, HeaderValue, Request, StatusCode},
    response::{AppendHeaders, IntoResponse},
    routing::post,
    Json, Router,
};

use crate::users::model::response::{LoginResponse, UserResponse};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::{
    error::ErrorResponse,
    response::WebResponse,
    state::AppState,
    users::auth::{make_access_token, make_refresh_token},
    WebResult,
};

use crate::users::model::{request::LoginRequest, validation::check_password};

#[utoipa::path(
    post,
    tag = "User",
    path = "/api/v1/user/login",
    request_body(
        content = LoginRequest,
        description = "Login Request",
    ),
    responses(
        (
            status = 200,
            description = "Login successfully",
            headers(
                ( "x-auth-access-token" = String, description = "Access token" ),
                ( "x-auth-refresh-token" = String, description = "Refresh token" )
            ),
            body = UserResponse,
            example = json!(
                {
                    "code": 200,
                    "message": "Login successfully",
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
                        "updatedAt": 1696932804946_i64
                    },
                    "error": ""
                }
            )
        )
    )
)]
pub fn login() -> Router<AppState> {
    async fn login_handler(
        State(AppState {
            mut user_service, ..
        }): State<AppState>,
        LoginRequest { username, password }: LoginRequest,
    ) -> WebResult {
        let found_user = user_service
            .get_user_by_login_info(username, password)
            .await?;

        let access_token = make_access_token(&found_user)?;
        let refresh_token = make_refresh_token(&found_user)?;

        let response = LoginResponse {
            user: found_user.into(),
            x_auth_access_token: access_token,
            x_auth_refresh_token: refresh_token,
        };
        // let response = (
        //     StatusCode::OK,
        //     AppendHeaders([
        //         (
        //             HeaderName::from_static("x-auth-access-token"),
        //             HeaderValue::from_str(&access_token).unwrap(),
        //         ),
        //         (
        //             HeaderName::from_static("x-auth-refresh-token"),
        //             HeaderValue::from_str(&refresh_token).unwrap(),
        //         ),
        //     ]),
        //     WebResponse::ok("Login successfully", UserResponse::from(found_user)),
        // );
        Ok(WebResponse::ok("Login successfully ", response))
    }
    Router::new().route("/login", post(login_handler))
}
