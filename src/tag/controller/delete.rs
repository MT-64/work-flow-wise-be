use axum::{extract::State, routing::delete, Router};

use crate::{
    extractors::param::ParamId,
    response::WebResponse,
    users::model::{loggedin::LoggedInUser, response::UserSelect},
    AppState, WebResult,
};

pub fn delete_tag() -> Router<AppState> {
    async fn delete_tag_handler(
        State(AppState { tag_service, .. }): State<AppState>,
        LoggedInUser(UserSelect {
            pk_user_id: user_id,
            ..
        }): LoggedInUser,
        ParamId(tag_id): ParamId,
    ) -> WebResult {
        let owned_tag = tag_service.get_owned_tag(tag_id, user_id).await?;
        tag_service.delete_tag(owned_tag.id).await?;
        Ok(WebResponse::ok("Delete tag successfully", ()))
    }
    Router::new().route("/delete/:tag_id", delete(delete_tag_handler))
}
