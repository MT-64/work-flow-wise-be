use axum::{extract::State, routing::post, Router};

use crate::{
    organize::model::{request::CreateOrganizeRequest, response::OrganizeResponse},
    prisma::{self, department},
    response::WebResponse,
    state::AppState,
    WebResult,
};

#[utoipa::path(
  post,
  tag = "Organize",
  path = "/api/v1/organize/create",
  request_body(
    content = CreateOrganizeRequest,
    description = "Create Organize Request",
  ),
  responses(
    (
      status = 201,
      description = "Organize created",
      body = OrganizeResponse,
      example = json! (
        {
          "code": 201,
          "message": "Created organize sucessfully",
          "data": {
            "id": "1w6ajp6l6gooi9g",
            "organizeId": "GFI",
            "managerId": "None",
            "name": "VBI"
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn create_organize() -> Router<AppState> {
    async fn create_organize_handler(
        State(AppState {
            organize_service, ..
        }): State<AppState>,
        CreateOrganizeRequest {
            owner_id,
            address,
            contact,
            name,
        }: CreateOrganizeRequest,
    ) -> WebResult {
        let mut params = vec![];

        let new_organize: OrganizeResponse = organize_service
            .create_organize(name, address, contact, owner_id, params)
            .await?
            .into();

        Ok(WebResponse::created(
            "Created organize sucessfully",
            OrganizeResponse::from(new_organize),
        ))
    }
    Router::new().route("/create", post(create_organize_handler))
}
