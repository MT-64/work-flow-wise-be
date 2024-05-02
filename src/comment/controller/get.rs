use axum::{
    extract::{Path, State},
    Router,
};

use crate::{
    comment::model::{CommentResponse, CommentTreeNodeResponse},
    response::WebResponse,
    state::AppState,
    WebResult,
};
use axum::routing::get;
use tracing::{info_span, instrument, Instrument};

#[utoipa::path(
  get,
  tag = "Comment",
  path = "/api/v1/comment/{post_id}",
  params(
    ("post_id" = String, Path, description = "Post ID"),
  ),
  responses(
    (
      status = 200,
      description = "Get commments by post id",
      body = Vec<CommentTreeNodeResponse>,
      example = json!(
            {
          "code": 200,
          "message": "Get all comments successfully",
          "data": [
            {
              "childers": [
                {
                  "childers": [],
                  "parentComment": {
                    "content": "You mean raw pointer ?",
                    "createdAt": 0,
                    "deletedAt": 0,
                    "id": "YTfXJIBPTPmv8YuWScln",
                    "isDeleted": false,
                    "parentId": "mwJb-rUCQGSy9p4MCiLD",
                    "postId": "7gQM24_VOoub43rdh_qi",
                    "score": 0,
                    "updatedAt": 0,
                    "userId": "_QGuhsRFu7aRAJOmYnHD"
                  },
                  "parentId": "mwJb-rUCQGSy9p4MCiLD",
                  "userId": "_QGuhsRFu7aRAJOmYnHD"
                },
                {
                  "childers": [],
                  "parentComment": {
                    "content": "raw pointer is so noob in rust",
                    "createdAt": 0,
                    "deletedAt": 0,
                    "id": "oRF604U36sCqhk5KXZln",
                    "isDeleted": false,
                    "parentId": "mwJb-rUCQGSy9p4MCiLD",
                    "postId": "7gQM24_VOoub43rdh_qi",
                    "score": 0,
                    "updatedAt": 0,
                    "userId": "_QGuhsRFu7aRAJOmYnHD"
                  },
                  "parentId": "mwJb-rUCQGSy9p4MCiLD",
                  "userId": "_QGuhsRFu7aRAJOmYnHD"
                }
              ],
              "parentComment": {
                "content": "What is pointer in rust ?",
                "createdAt": 0,
                "deletedAt": 0,
                "id": "mwJb-rUCQGSy9p4MCiLD",
                "isDeleted": false,
                "parentId": null,
                "postId": "7gQM24_VOoub43rdh_qi",
                "score": 0,
                "updatedAt": 0,
                "userId": "_QGuhsRFu7aRAJOmYnHD"
              },
              "parentId": "",
              "userId": "_QGuhsRFu7aRAJOmYnHD"
            }
          ],
          "error": ""
}
          )
      )
    ),
)]

pub fn get_comments_by_post() -> Router<AppState> {
    async fn get_comments_by_post_handler(
        State(AppState {
            mut comment_service,
            ..
        }): State<AppState>,
        Path(post_id): Path<String>,
    ) -> WebResult {
        let comments: Vec<CommentTreeNodeResponse> = comment_service
            .get_comment_post_call(post_id)
            .await?
            .into_iter()
            .map(|c| c.into())
            .collect();

        Ok(WebResponse::ok("Get all comments successfully", comments))
    }
    Router::new().route("/:post_id", get(get_comments_by_post_handler))
}

#[utoipa::path(
  get,
  tag = "Comment",
  path = "/api/v1/comment/{post_id}/{comment_id}",
  params(
    ("post_id" = String, Path, description = "Post ID"),
    ("comment_id" = String, Path, description = "Comment ID"),
  ),
  responses(
    (
      status = 200,
      description = "Get comment by id",
      body = CommentResponse,
      )
    ),
)]
pub fn get_comment_by_id() -> Router<AppState> {
    async fn get_comment_by_id_handler(
        State(AppState {
            mut comment_service,
            ..
        }): State<AppState>,
        Path((post_id, comment_id)): Path<(String, String)>,
    ) -> WebResult {
        let comment: CommentResponse = comment_service
            .get_comment_by_id(comment_id, post_id)
            .await?
            .into();

        Ok(WebResponse::ok("Get comment successfully", comment))
    }

    Router::new().route("/:post_id/:comment_id", get(get_comment_by_id_handler))
}
