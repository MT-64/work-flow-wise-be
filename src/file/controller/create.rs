use axum::{extract::State, routing::post, Router};
use axum_typed_multipart::{FieldData, FieldMetadata};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    file::model::create::{CreateFileRequest, UploadRequest},
    helpers::id::generate_id,
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

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct FileUploadedResponse {
    pub fullname: String,
    pub virtual_path: String,
}

#[utoipa::path(
  post,
  tag = "File_processing",
  path = "/api/v1/file/upload",
  request_body(
    content = UploadRequest,
    content_type = "multipart/form-data",
    description = "Upload file Request",
  ),
  responses(
    (
      status = 201,
      description = "Upload file success",
      body = FileUploadedResponse,
    ),
  )
)]
pub fn upload_file() -> Router<AppState> {
    async fn create_file_handler(
        State(AppState { storage, .. }): State<AppState>,
        // LoggedInUser(UserSelect {
        //     pk_user_id: user_id,
        //     ..
        // }): LoggedInUser,
        UploadRequest {
            file:
                FieldData {
                    metadata: FieldMetadata { file_name, .. },
                    contents,
                },
        }: UploadRequest,
    ) -> WebResult {
        let file_name: String = file_name.expect("Invalid file");

        let (name, extension) = file_name.split_once('.').expect("Cannot get extension");
        let old_fullpath = format!("{}.{}", name, extension);
        let new_fullpath = format!("{}.{}", generate_id(), extension.to_string());
        storage.create_file(&new_fullpath, contents).await?;

        Ok(WebResponse::created(
            "Created a new file",
            FileUploadedResponse {
                fullname: old_fullpath,
                virtual_path: new_fullpath,
            },
        ))
    }
    Router::new().route("/upload", post(create_file_handler))
}
