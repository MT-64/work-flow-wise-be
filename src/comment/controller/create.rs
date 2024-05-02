use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, State},
    http::Request,
    routing::post,
    Json, Router,
};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::{
    comment::model::{CommentResponse, DbComment},
    error::ErrorResponse,
    helpers::id::generate_id,
    response::WebResponse,
    state::AppState,
    users::model::loggedin::LoggedInUser,
    WebResult,
};

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct AddCommentToCommentRequest {
    parent_id: Option<String>,
    post_id: String,
    content: String,
}

#[async_trait]
impl FromRequest<AppState, Body> for AddCommentToCommentRequest {
    type Rejection = ErrorResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(req) = Json::<AddCommentToCommentRequest>::from_request(req, state).await?;

        let AddCommentToCommentRequest { .. } = &req;

        req.validate()?;

        Ok(req)
    }
}

#[utoipa::path(
  post,
  tag = "Comment",
  path = "/api/v1/comment/create_comment",
  request_body(
    content = AddCommentToCommentRequest,
    description = "Create Comment Request",
  ),
  responses(
    (
      status = 201,
      description = "Add to comment",
      body = CommentResponse,
      example = json! (
        {
          "code": 201,
          "message": "Add to comment successfully",
          "data": {
          },
          "error": ""
        }
      )
    ),
  )
)]

pub fn add_to_comment() -> Router<AppState> {
    async fn add_to_comment_handler(
        State(AppState {
            mut comment_service,
            ..
        }): State<AppState>,
        LoggedInUser(user): LoggedInUser,
        AddCommentToCommentRequest {
            parent_id,
            post_id,
            content,
        }: AddCommentToCommentRequest,
    ) -> WebResult {
        let created_comment: CommentResponse = comment_service
            .create_comment(parent_id, post_id, content, user.pk_user_id)
            .await?
            .into();

        Ok(WebResponse::created(
            "Add comment to comment successfully",
            created_comment,
        ))
    }
    Router::new().route("/create_comment", post(add_to_comment_handler))
}
