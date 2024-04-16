use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;

use crate::{error::ErrorResponse, state::AppState, users::auth::decode_access_token};

use super::response::{UserSelect, UserSelectWithPassword};
use crate::prisma::Role;

pub struct LoggedInUser(pub UserSelect);

#[async_trait]
impl FromRequestParts<AppState> for LoggedInUser {
    type Rejection = ErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(authorization)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| ErrorResponse::Permissions)?;

        let id = decode_access_token(authorization.token().to_string())?;

        let user = state.user_service.get_user_by_id(id).await?;

        Ok(LoggedInUser(user.clone()))
    }
}
pub struct LoggedInAdmin(pub UserSelect);

#[async_trait]
impl FromRequestParts<AppState> for LoggedInAdmin {
    type Rejection = ErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let LoggedInUser(user) = LoggedInUser::from_request_parts(parts, state).await?;

        if user.role != Role::Admin {
            return Err(ErrorResponse::Permissions);
        }

        Ok(LoggedInAdmin(user))
    }
}
