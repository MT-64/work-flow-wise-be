use axum::{extract::State, routing::get, Router};

use crate::{
    file::model::query::FileQuery,
    response::WebResponse,
    users::model::{loggedin::LoggedInUser, response::UserSelect},
    AppState, WebResult,
};

/*
    On the handlers side

    We only have to deal with owner_id, parent, and visiblity
*/

pub fn get_shared_files() -> Router<AppState> {
    async fn get_shared_files_handler(
        State(AppState { file_service, .. }): State<AppState>,
        LoggedInUser(UserSelect {
            pk_user_id: user_id,
            ..
        }): LoggedInUser,
        FileQuery {
            id,
            parent_folder_id,
            filename,
            extension,
            created_at,
            updated_at,
            ..
        }: FileQuery,
    ) -> WebResult {
        let shared_files = file_service
            .get_files_shared_to_user_id(
                user_id,
                id,
                parent_folder_id,
                filename,
                extension,
                created_at,
                updated_at,
            )
            .await?;

        Ok(WebResponse::ok(
            "Get all shared to me files successfully",
            shared_files,
        ))
    }
    Router::new().route("/shared", get(get_shared_files_handler))
}
