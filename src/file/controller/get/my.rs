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
#[utoipa::path(
    get,
    tag = "File",
    path = "/api/v1/file/my",
    params(
      ("id" = inline(Option<i64>), Query, description = "id"),
      ("parent_folder_id" = inline(Option<i32>), Query, description = "parent folder id"),
      ("filename" = inline(Option<String>), Query, description = "filename"),
      ("extension" = inline(Option<String>), Query, description = "extension"),
      ("visibility" = inline(Option<bool>), Query, description = "visibility"),
      ("createdAt" = inline(Option<i64>), Query, description = "File created at"),
      ("updatedAt" = inline(Option<i64>), Query, description = "File updated at"),
    ),
    responses(
        (
            status = 200,
            description = "Get all personal files",
            body = Vec<File>, // Replace with your actual file data structure
            example = json!({
                "message": "Get all personal files successfully",
                "data": [
                    // ... Example file objects
                ],
                "error": ""
            }),
        ),
    ),
)]
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
