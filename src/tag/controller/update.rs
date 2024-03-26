use axum::{extract::State, routing::put, Router};

use crate::{
    extractors::param::ParamId,
    response::WebResponse,
    tag::model::update::UpdateTagRequest,
    users::model::{loggedin::LoggedInUser, response::UserSelect},
    AppState, WebResult,
};

pub fn update_tag() -> Router<AppState> {
    async fn update_tag_handler(
        State(AppState { tag_service, .. }): State<AppState>,
        LoggedInUser(UserSelect {
            pk_user_id: user_id,
            ..
        }): LoggedInUser,
        ParamId(tag_id): ParamId,
        UpdateTagRequest { tag_name }: UpdateTagRequest,
    ) -> WebResult {
        let old_tag = tag_service.get_owned_tag(tag_id, user_id).await?;
        let updated_tag = tag_service.update_tag(old_tag.id, tag_name).await?;
        Ok(WebResponse::ok("Update tag successfully", updated_tag))
    }
    Router::new().route("/update/:tag_id", put(update_tag_handler))
}
