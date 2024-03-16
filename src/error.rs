use crate::{
    helpers::validation::{extract_validation_error, validation_message},
    response::WebResponse,
};

use aws_sdk_s3::{
    error::{BuildError, SdkError},
    operation::{
        copy_object::CopyObjectError, delete_object::DeleteObjectError,
        delete_objects::DeleteObjectsError, get_object::GetObjectError,
        list_objects_v2::ListObjectsV2Error, put_object::PutObjectError,
    },
};
use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    response::{IntoResponse, Response},
};
use axum_typed_multipart::TypedMultipartError;
use prisma_client_rust::query_core::error;
use prisma_client_rust::{
    prisma_errors::query_engine::{
        ConstraintViolation, RecordNotFound, TableDoesNotExist, UniqueKeyViolation,
    },
    QueryError,
};
use thiserror::Error;
use validator::{ValidationError, ValidationErrors};

pub fn match_query_error(error: QueryError) -> Response {
    if error.is_prisma_error::<UniqueKeyViolation>() {
        WebResponse::conflict(
            "Conflict data",
            "The provided data is already exists, please try another",
        )
    } else if error.is_prisma_error::<ConstraintViolation>() {
        WebResponse::bad_request(
            "Constraint violated",
            "A constraint in the database has been violated",
        )
    } else if error.is_prisma_error::<RecordNotFound>() {
        WebResponse::not_found(
            "Not found data",
            "The information provided could not be found in the database",
        )
    } else if error.is_prisma_error::<TableDoesNotExist>() {
        WebResponse::internal_error(
            "Table does not exists",
            "The database has not yet been initialized",
        )
    } else {
        WebResponse::internal_error("Unknown error", error)
    }
}

#[derive(Debug, Error)]
pub enum ErrorResponse {
    #[error("Query error")]
    DatabaseQuery(#[from] QueryError),

    #[error("Json parsing error")]
    JsonParsing(#[from] JsonRejection),

    #[error("Query parsing error")]
    QueryParsing(#[from] QueryRejection),

    #[error("Path parsing error")]
    Path(#[from] PathRejection),

    #[error("Multipart error")]
    Multipart(#[from] TypedMultipartError),

    // Authentication errors
    #[error("JWT error")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Permissions error")]
    Permissions,

    #[error("Not Found")]
    NotFound,

    #[error("Password hashing error")]
    PasswordHashing(#[from] argon2::password_hash::Error),

    #[error("Single invalid field")]
    SingleInvalidField(#[from] ValidationError),

    #[error("Multiple invalid field")]
    MultipleInvalidField(#[from] ValidationErrors),

    #[error("Body no content")]
    NoContent,

    #[error("Forbidden")]
    Forbidden,

    #[error("Download error")]
    Download(#[from] std::io::Error),

    #[error("Zip error")]
    Zip(#[from] zip::result::ZipError),

    /*
        AWS errors
    */
    #[error("Upload file error")]
    PutObject(#[from] SdkError<PutObjectError>),

    #[error("Get single file error")]
    GetObject(#[from] SdkError<GetObjectError>),

    #[error("List all files error")]
    ListObject(#[from] SdkError<ListObjectsV2Error>),

    #[error("Copy file error")]
    CopyObject(#[from] SdkError<CopyObjectError>),

    #[error("Delete file error")]
    DeleteObject(#[from] SdkError<DeleteObjectError>),

    #[error("Delete files error")]
    DeleteObjects(#[from] SdkError<DeleteObjectsError>),

    #[error("Build ObjectIdentifier error")]
    BuildObjectIdentifier(#[from] BuildError),
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            ErrorResponse::DatabaseQuery(e) => match_query_error(e),

            ErrorResponse::JsonParsing(e) => WebResponse::bad_request("Invalid request body", e),

            ErrorResponse::QueryParsing(e) => WebResponse::bad_request("Invalid query provided", e),

            ErrorResponse::Multipart(e) => WebResponse::bad_request("Invalid form data", e),

            ErrorResponse::Jwt(_) => {
                WebResponse::bad_request("Authentication error", "Please try logging in again")
            }

            ErrorResponse::Permissions => WebResponse::forbidden(
                "Permissions error",
                "You don't have permissions to see this page, please try logging in again",
            ),

            ErrorResponse::NotFound => WebResponse::not_found(
                "Not found",
                "The value provided for query could not be found",
            ),

            ErrorResponse::PasswordHashing(e) => {
                WebResponse::internal_error("Cannot hash the password", e)
            }

            ErrorResponse::SingleInvalidField(e) => {
                WebResponse::bad_request("One of the request fields might be incorrect", e)
            }

            ErrorResponse::MultipleInvalidField(e) => WebResponse::bad_request(
                "Multiple request fields are invalid",
                extract_validation_error(&e),
            ),

            ErrorResponse::NoContent => {
                WebResponse::bad_request("No content error", "Your body request must have content")
            }

            ErrorResponse::Path(e) => WebResponse::bad_request(
                "Path error",
                format!(
                    "The value in the path parameter cannot be used. Error: {}",
                    e
                ),
            ),

   /*
                AWS S3 errors
            */
            ErrorResponse::GetObject(_) => WebResponse::bad_request(
                "Get file error",
                "This maybe due to the information provided was incorrect",
            ),

            ErrorResponse::PutObject(_) => WebResponse::bad_request(
                "Cannot upload file",
                "This is maybe due to the file is in wrong format or too large for upload",
            ),

            ErrorResponse::CopyObject(_) => WebResponse::internal_error(
                "Cannot copy file",
                "This is maybe due to the storage server unable to copy, please try again later",
            ),

            ErrorResponse::ListObject(_) => WebResponse::internal_error(
                "Cannot get all files",
                "Probably database error, please try again later",
            ),

            ErrorResponse::DeleteObject(_) => WebResponse::internal_error(
                "Cannot delete file", 
                "This is due to the database have problems that prevent the file from being deleted"
            ),
            ErrorResponse::DeleteObjects(_) => WebResponse::internal_error(
                "Cannot delete multiple files",
                "This is due to database error, which make some files couldn't be deleted"
            ),
            ErrorResponse::BuildObjectIdentifier(_) => WebResponse::internal_error(
                "Object ID error", 
                "This is due to database error, which causes id of some file are invalid"
            ),
            ErrorResponse::Forbidden => WebResponse::forbidden(
                "Forbidden",
                "You do not have permissions to perform this action",
            ),
            ErrorResponse::Download(e) => WebResponse::internal_error("The download process failed", e),
            ErrorResponse::Zip(e) => WebResponse::internal_error("The zip process failed", e)

        }
    }
}
