use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, FromRequestParts, Query},
    http::request::Parts,
    http::Request,
    Json,
};
use is_empty::IsEmpty;
use serde::Deserialize;
use validator::Validate;

use crate::{error::ErrorResponse, helpers::validation::validation_message, state::AppState};

use super::{
    response::Role,
    validation::{check_password, check_username},
};

use utoipa::ToSchema;

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserRequest {
    #[validate(custom = "check_password")]
    pub password: String,

    #[validate(custom = "check_password")]
    pub confirm_password: String,
}

#[async_trait]
impl FromRequest<AppState, Body> for DeleteUserRequest {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<DeleteUserRequest>::from_request(req, state).await?;
        let DeleteUserRequest {
            password,
            confirm_password,
        } = &body;
        if password != confirm_password {
            return Err(validation_message("Passwords do not match").into());
        }
        Ok(body)
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    #[validate(custom = "check_username")]
    pub username: String,

    #[validate(email(message = "Invalid email"))]
    pub email: Option<String>,

    #[validate(custom = "check_password")]
    pub password: String,

    #[validate(custom = "check_password")]
    pub confirm_password: String,
}

#[async_trait]
impl FromRequest<AppState, Body> for CreateUserRequest {
    type Rejection = ErrorResponse;
    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<CreateUserRequest>::from_request(req, state).await?;
        let CreateUserRequest {
            password,
            confirm_password,
            ..
        } = &body;
        if password != confirm_password {
            return Err(validation_message("Passwords are not equal").into());
        }
        Ok(body)
    }
}

#[derive(Deserialize, Validate, IsEmpty, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    #[validate(custom = "check_username")]
    pub username: Option<String>,

    #[validate(email(message = "Invalid email form"))]
    pub email: Option<String>,

    pub role: Option<crate::prisma::Role>,

    #[validate(custom = "check_password")]
    #[is_empty(if = "String::is_empty")]
    pub password: String,

    #[validate(custom = "check_password")]
    pub new_password: Option<String>,

    #[validate(custom = "check_password")]
    pub confirm_new_password: Option<String>,

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<crate::prisma::Gender>,
    pub introduction_brief: Option<String>,
    pub image: Option<String>,
}

#[async_trait]
impl FromRequest<AppState, Body> for UpdateUserRequest {
    type Rejection = ErrorResponse;
    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<UpdateUserRequest>::from_request(req, state).await?;

        // Just return no content if the body is empty
        if body.is_empty() {
            return Err(ErrorResponse::NoContent);
        }

        let UpdateUserRequest {
            new_password,
            confirm_new_password,
            username,
            email,
            password,
            role,
            first_name,
            last_name,
            gender,
            introduction_brief,
            image,
            ..
        } = &body;

        // Ensure that both new password and confirm new password fields are equal
        if (new_password.is_some() && confirm_new_password.is_none())
            || (new_password.is_none() && confirm_new_password.is_some())
        {
            return Err(validation_message("Both field newPassword and confirmNewPassword must exists together, or omit them both").into());
        }
        if let (Some(new_password), Some(confirm_new_password)) =
            (new_password, confirm_new_password)
        {
            if new_password != confirm_new_password {
                return Err(validation_message("Both passwords are not the same").into());
            }
        }

        Ok(body)
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(custom = "check_username")]
    pub username: String,

    #[validate(custom = "check_password")]
    pub password: String,
}

#[async_trait]
impl FromRequest<AppState, Body> for LoginRequest {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(req) = Json::<LoginRequest>::from_request(req, state).await?;
        req.validate()?;
        Ok(req)
    }
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryRequest {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub id: Option<String>,
    pub department_id: Option<String>,
    pub organize_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

#[async_trait]
impl FromRequestParts<AppState> for UserQueryRequest {
    type Rejection = ErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Query(user_query) = Query::<UserQueryRequest>::from_request_parts(parts, state).await?;

        Ok(user_query)
    }
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddMultipleUserToDepartment {
    pub list_user: Vec<String>,
    pub department_id: String,
}

#[async_trait]
impl FromRequest<AppState, Body> for AddMultipleUserToDepartment {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<AddMultipleUserToDepartment>::from_request(req, state).await?;
        let AddMultipleUserToDepartment {
            list_user,
            department_id,
        } = &body;
        Ok(body)
    }
}
#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddMultipleUserToOrg {
    pub list_user: Vec<String>,
    pub org_id: String,
}

#[async_trait]
impl FromRequest<AppState, Body> for AddMultipleUserToOrg {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<AddMultipleUserToOrg>::from_request(req, state).await?;
        let AddMultipleUserToOrg { list_user, org_id } = &body;
        Ok(body)
    }
}
