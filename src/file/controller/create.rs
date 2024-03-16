use axum::{extract::State, routing::post, Router};
use axum_typed_multipart::{FieldData, FieldMetadata};

use crate::{
    file::model::create::CreateFileRequest,
    prisma::{folder, Visibility},
    response::WebResponse,
    users::model::{loggedin::LoggedInUser, response::UserSelect},
    validation::validation_message,
    AppState, WebResult,
};

pub fn create_file() -> Router<AppState> {
    async fn create_file_handler(
        State(AppState {
            file_service,
            folder_service,
            storage,
            ..
        }): State<AppState>,
        LoggedInUser(UserSelect {
            pk_user_id: user_id,
            ..
        }): LoggedInUser,
        CreateFileRequest {
            parent,
            visibility,
            file:
                FieldData {
                    metadata: FieldMetadata { file_name, .. },
                    contents,
                },
        }: CreateFileRequest,
    ) -> WebResult {
        let Some(file_name) = file_name else {
            return Err(validation_message(
                "The uploaded file should have a name and an extension",
            )
            .into());
        };

        let starting_point = match parent {
            Some(parent) => vec![folder::id::equals(parent)],
            None => vec![
                folder::parent_folder_id::equals(None),
                folder::owner_id::equals(user_id.clone()),
            ],
        };

        let parent_folder = folder_service
            .get_folder_by_user_id(starting_point, user_id)
            .await?;

        let new_file = file_service
            .create_file(
                parent_folder.id,
                file_name,
                visibility.unwrap_or(Visibility::Public),
                parent_folder.owner.pk_user_id,
            )
            .await?;

        /*
            Creates a new file in the S3 storage with the new file id
            Format: new-file-id.extension
        */
        storage
            .create_file(
                &format!("{}.{}", new_file.id, new_file.extension.to_string()),
                contents,
            )
            .await?;
        /*
            Also creates a new folder with the new file id
            Format: new-file-id/
        */
        storage.create_folder(&format!("{}/", new_file.id)).await?;

        Ok(WebResponse::created("Created a new file", new_file))
    }
    Router::new().route("/create", post(create_file_handler))
}
