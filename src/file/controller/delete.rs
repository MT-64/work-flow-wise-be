use axum::{
    extract::{Path, State},
    routing::delete,
    Router,
};

use crate::{
    extractors::param::ParamId,
    file_version::model::FileVersionSelect,
    prisma::file,
    response::WebResponse,
    users::model::{loggedin::LoggedInUser, response::UserSelect},
    AppState, WebResult,
};

pub fn delete_file() -> Router<AppState> {
    async fn delete_file_handler(
        State(AppState {
            file_service,
            storage,
            ..
        }): State<AppState>,
        ParamId(file_id): ParamId,
        LoggedInUser(UserSelect {
            pk_user_id: user_id,
            ..
        }): LoggedInUser,
    ) -> WebResult {
        let target_file = file_service
            .get_file_by_user_id(vec![file::id::equals(file_id)], user_id)
            .await?;

        let deleted_file = file_service.delete_file(target_file.id).await?;

        storage
            .delete_file(&format!(
                "{}.{}",
                deleted_file.id,
                deleted_file.extension.to_string()
            ))
            .await?;
        storage
            .delete_folder(&format!("{}/", deleted_file.id))
            .await?;

        Ok(WebResponse::ok("Deleted file successfully", ()))
    }
    Router::new().route("/delete/:file_id", delete(delete_file_handler))
}

pub fn delete_file_version() -> Router<AppState> {
    async fn delete_file_version_handler(
        State(AppState {
            file_service,
            file_version_service,
            storage,
            ..
        }): State<AppState>,
        ParamId(file_id): ParamId,
        Path(version_number): Path<i64>,
        LoggedInUser(UserSelect {
            pk_user_id: user_id,
            ..
        }): LoggedInUser,
    ) -> WebResult {
        let target_file = file_service
            .get_file_by_user_id(vec![file::id::equals(file_id)], user_id)
            .await?;

        let FileVersionSelect {
            file: deleted_file_version,
            version_number: deleted_version,
        } = file_version_service
            .delete_version_from_file(target_file.id, version_number)
            .await?;

        storage
            .delete_file(&format!(
                "{}/{}.{}",
                deleted_file_version.id,
                deleted_version,
                deleted_file_version.extension.to_string()
            ))
            .await?;

        Ok(WebResponse::ok("Deleted file version successfully", ()))
    }
    Router::new().route(
        "/delete/:file_id/:version_number",
        delete(delete_file_version_handler),
    )
}
