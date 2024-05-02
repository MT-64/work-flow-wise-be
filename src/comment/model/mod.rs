use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::helpers::id::generate_id;

#[derive(Debug)]
pub struct CommentTreeNode {
    pub comment: DbComment,
    pub user_id: String,
    pub children: Vec<CommentTreeNode>,
    pub parent_comment_id: String,
}
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommentTreeNodeResponse {
    pub parent_comment: CommentResponse,
    pub user_id: String,
    pub parent_id: String,
    pub childers: Vec<CommentTreeNodeResponse>,
}

impl From<CommentTreeNode> for CommentTreeNodeResponse {
    fn from(
        CommentTreeNode {
            comment,
            user_id,
            parent_comment_id,
            children,
        }: CommentTreeNode,
    ) -> Self {
        Self {
            parent_comment: comment.into(),
            user_id,
            parent_id: parent_comment_id,
            childers: children.into_iter().map(|child| child.into()).collect(),
        }
    }
}

#[derive(Debug, Default, Clone, FromRow)]
pub struct DbComment {
    #[sqlx(rename = "pk_comment_id")]
    pub id: String,
    pub parent_comment_id: Option<String>,
    pub post_id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: i64,
    pub deleted_at: i64,
    pub updated_at: i64,
    pub is_deleted: bool,
}

#[derive(Debug, Default, Clone, FromRow)]
pub struct PostData {
    #[sqlx(rename = "pk_post_id")]
    pub post_id: String,
    pub vote_value: i32,
    pub comment_count: u32,
    pub date_string: String,
}

#[derive(Debug, Default, Clone, FromRow)]
pub struct DbPost {
    #[sqlx(rename = "pk_post_id")]
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub created_at: i64,
}

#[derive(Debug, Default, Clone, FromRow)]
pub struct DbPostQuery {
    pub offset: Option<i64>,
    pub limit: Option<i32>,
    pub start_at: Option<i64>,
    pub end_at: Option<i64>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommentResponse {
    pub id: String,
    pub parent_id: Option<String>,
    pub post_id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: i64,
    pub deleted_at: i64,
    pub updated_at: i64,
    pub is_deleted: bool,
}

impl From<DbComment> for CommentResponse {
    fn from(
        DbComment {
            id,
            parent_comment_id,
            post_id,
            user_id,
            content,
            created_at,
            deleted_at,
            updated_at,
            is_deleted,
        }: DbComment,
    ) -> Self {
        Self {
            id,
            parent_id: parent_comment_id,
            post_id,
            user_id,
            content,
            created_at,
            deleted_at,
            updated_at,
            is_deleted,
        }
    }
}
