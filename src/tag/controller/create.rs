use axum::{extract::State, routing::post, Router};

use crate::{
    response::WebResponse,
    tag::model::create::CreateTagRequest,
    users::model::{loggedin::LoggedInUser, response::UserSelect},
    AppState, WebResult,
};

pub fn create_tag() -> Router<AppState> {
    async fn create_tag_handler(
        State(AppState { tag_service, .. }): State<AppState>,
        LoggedInUser(UserSelect {
            pk_user_id: user_id,
            ..
        }): LoggedInUser,
        CreateTagRequest { tag_name }: CreateTagRequest,
    ) -> WebResult {
        let new_tag = tag_service.create_tag(tag_name, user_id).await?;
        Ok(WebResponse::created(
            "Created new tag successfully",
            new_tag,
        ))
    }
    Router::new().route("/create", post(create_tag_handler))
}
