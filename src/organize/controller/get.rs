use crate::{organize::model::request::OrganizeQueryRequest};
use axum::{extract::{State, Path}, routing::get, Router};
use chrono::DateTime;
use prisma_client_rust::query_core::schema_builder::constants::filters;
use crate::prisma::organize;

use crate::{
    extractors::param::ParamId,
    organize::model::response::OrganizeResponse,
    response::WebResponse,
    state::AppState,
    WebResult,
};

#[utoipa::path(
  get,
  tag = "Organize",
  path = "/api/v1/organize",
  params(
    ("offset" = inline(Option<i64>), Query, description = "Starting point"),
    ("limit" = inline(Option<i32>), Query, description = "Limit"),
    ("id" = inline(Option<String>), Query, description = "Organize id"),
    ("name" = inline(Option<String>), Query, description = "Obj name"),
    ("owner_id" = inline(Option<String>), Query, description = "owner id"),
  ),
  responses(
    (
      status = 200,
      description = "Get organizes",
      body = Vec<OrganizeResponse>,
      example = json!(
                {
          "code": 200,
          "message": "Get organizes successfully",
          "data": [
            {
              "id": "1w6ajp6l6gooi9g",
              "organizeId": "GFI",
              "managerId": "None",
              "name": "VBI"
            },
            {
              "id": "ojw8a7ibg1ah5gj",
              "organizeId": "string",
              "managerId": "string",
              "name": "string"
            }
          ],
          "error": ""
        }
      )
    ),
  )
)]
pub fn get_organizes() -> Router<AppState> {
    async fn get_organizes_handler(
        State(AppState { organize_service, .. }): State<AppState>,
        OrganizeQueryRequest { offset, limit, id, name, owner_id }: OrganizeQueryRequest
    ) -> WebResult {
        let offset = offset.unwrap_or(0);

        let limit = match limit {
            Some(limit) => match limit {
                0..=50 => limit,
                _ => 10,
            },
            None => 10,
        };

        let mut filters = vec![];

        if let Some(id) = id {
            filters.push(organize::pk_organize_id::equals(id));
        }

        if let Some(name) = name {
            filters.push(organize::name::equals(name));
        }

        if let Some(owner_id) = owner_id {
            filters.push(organize::owner_id::equals(owner_id));
        }

        let orgs: Vec<OrganizeResponse> = organize_service
            .get_organizes(filters, offset, limit)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();
        Ok(WebResponse::ok("Get orgs successfully", orgs))
    }
    Router::new().route("/", get(get_organizes_handler))
}

#[utoipa::path(
  get,
  tag = "Organize",
  path = "/api/v1/organize/{org_id}",
  params(
    ("org_id" = String, Path, description = "Organize ID")
  ),
  responses(
    (
      status = 201,
      description = "Get organize by organize id",
      body = OrganizeResponse,
      example = json! (
               {
          "code": 200,
          "message": "Get organize by id successfully",
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
pub fn get_organize() -> Router<AppState> {
    async fn get_organize_handler(
        State(AppState { organize_service, .. }): State<AppState>,
        Path(org_id): Path<String>,
    ) -> WebResult {
        let org: OrganizeResponse = organize_service
            .get_organize_by_id(org_id)
            .await?.into();
        Ok(WebResponse::ok("Get organize by id successfully", org))
    }
    Router::new().route("/:org_id", get(get_organize_handler))
}
