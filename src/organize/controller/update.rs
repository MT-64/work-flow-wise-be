use axum::{
    extract::{Path, State},
    routing::put,
    Router,
};
use chrono::DateTime;

use crate::{
    helpers::validation::validation_message,
    organize::model::{request::UpdateOrganizeRequest, response::OrganizeResponse},
    response::WebResponse,
    state::AppState,
    users::model::loggedin::LoggedInUser,
    WebResult,
};

use crate::prisma::organize;

#[utoipa::path(
  put,
  tag = "Organize",
  path = "/api/v1/organize/update/{org_id}",
  params(
    ("org_id" = String, Path, description = "Department ID")
  ),

  request_body(
    content = UpdateOrganizeRequest,
  content_type = "multipart/form-data",
    description = "Update department request",
  ),
  responses(
    (
      status = 200,
      description = "Updated organize successfully",
      body = OrganizeResponse,
      example = json!(
        {
          "code": 200,
          "message": "Updated department successfully",
          "data": {
                     },
          "error": ""
        }
      )
    )
  )
)]
pub fn update_organize() -> Router<AppState> {
    async fn update_organize_handler(
        State(AppState {
            organize_service, ..
        }): State<AppState>,
        Path(org_id): Path<String>,
        LoggedInUser(user): LoggedInUser,
        UpdateOrganizeRequest {
            owner_id,
            address,
            contact,
            name,
        }: UpdateOrganizeRequest,
    ) -> WebResult {
        let mut changes = vec![];

        if let Some(name) = name {
            changes.push(organize::name::set(name));
        }

        if let Some(address) = address {
            changes.push(organize::address::set(address))
        }

        if let Some(contact) = contact {
            changes.push(organize::contact::set(contact))
        }

        if let Some(owner_id) = owner_id {
            changes.push(organize::owner_id::set(owner_id))
        }

        let updated_organize: OrganizeResponse = organize_service
            .update_organize(org_id, changes)
            .await?
            .into();
        Ok(WebResponse::ok(
            "Update organize successfully",
            updated_organize,
        ))
    }
    Router::new().route("/update/:org_id", put(update_organize_handler))
}
