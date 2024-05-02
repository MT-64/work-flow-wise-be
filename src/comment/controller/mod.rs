use self::{
    create::add_to_comment,
    get::{get_comment_by_id, get_comments_by_post},
};
use crate::state::AppState;
use axum::Router;

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub fn comment_routes() -> Router<AppState> {
    Router::new().nest(
        "/api/v1/comment",
        Router::new()
            .merge(get_comment_by_id())
            .merge(get_comments_by_post())
            .merge(add_to_comment()),
    )
}
