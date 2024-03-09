use axum::{extract::State, routing::get, Router};

use crate::{response::WebResponse, tag::model::query::TagQuery, AppState, WebResult};

pub fn get_tags() -> Router<AppState> {
    async fn get_tags_handler(
        State(AppState { tag_service, .. }): State<AppState>,
        TagQuery {
            id: tag_id,
            tag_name,
            owner_id,
            file_id,
            folder_id,
        }: TagQuery,
    ) -> WebResult {
        let tags = tag_service
            .get_tags(tag_id, tag_name, owner_id, file_id, folder_id)
            .await?;

        Ok(WebResponse::ok("Get all tags success", tags))
    }
    Router::new().route("/", get(get_tags_handler))
}
