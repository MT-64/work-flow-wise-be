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

pub fn get_my_files() -> Router<AppState> {
    async fn get_my_files_handler(
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
            visibility,
            created_at,
            updated_at,
            ..
        }: FileQuery,
    ) -> WebResult {
        let my_files = file_service
            .get_child_files_from_folders(
                id,
                Some(user_id),
                parent_folder_id,
                filename,
                extension,
                visibility,
                created_at,
                updated_at,
            )
            .await?;

        Ok(WebResponse::ok(
            "Get all personal files successfully",
            my_files,
        ))
    }
    Router::new().route("/my", get(get_my_files_handler))
}
