use axum::{
    extract::{Path, State},
    routing::post,
    Router,
};

use crate::{
    objectives::model::{request::CreateObjRequest, response::ObjectiveResponse},
    prisma::{self, objective, user},
    response::WebResponse,
    state::AppState,
    WebResult,
};

#[utoipa::path(
  post,
  tag = "Objective",
  path = "/api/v1/objective/{obj_id}/add_to_organize/{org_id}",
  params(
    ("obj_id" = String, Path, description = "Objective ID"),
    ("org_id" = String, Path, description = "Org ID")
  ),
  responses(
    (
      status = 201,
      description = "Add objective to organize successfully",
      body = WebResponse,
      example = json! (
        {
          "code": 201,
          "message": "Add objective to organize successfully",
          "data": null,
          "error": ""
        }
      )
    ),
  )
)]
pub fn add_to_organize() -> Router<AppState> {
    async fn add_to_organize_handler(
        State(AppState {
            obj_service,
            user_service,
            notification_service,
            ..
        }): State<AppState>,
        Path((obj_id, org_id)): Path<(String, String)>,
    ) -> WebResult {
        obj_service
            .add_to_org(obj_id.clone(), org_id.clone())
            .await?;

        let obj = obj_service.get_obj_by_id(obj_id).await?;

        let users = user_service
            .get_users(vec![user::organize_id::equals(Some(org_id))], 0, 100)
            .await?;

        let message = format!(r#"Mục tiêu mới {} được gán cho bạn"#, obj.name);

        for user in users {
            notification_service
                .create_noti(user.pk_user_id, message.clone(), vec![])
                .await?;
        }

        Ok(WebResponse::created(
            "Add objective to organize sucessfully",
            (),
        ))
    }
    Router::new().route(
        "/:obj_id/add_to_organize/:org_id",
        post(add_to_organize_handler),
    )
}

#[utoipa::path(
  post,
  tag = "Objective",
  path = "/api/v1/objective/{obj_id}/remove_from_org/{org_id}",
  params(
    ("obj_id" = String, Path, description = "Objective ID"),
    ("org_id" = String, Path, description = "Org ID")
  ),
  responses(
    (
      status = 201,
      description = "Add objective to department successfully",
      body = WebResponse,
      example = json! (
        {
          "code": 201,
          "message": "Add objective to department successfully",
          "data": null,
          "error": ""
        }
      )
    ),
  )
)]
pub fn remove_from_org() -> Router<AppState> {
    async fn remove_from_org_handler(
        State(AppState {
            obj_service,
            user_service,
            notification_service,
            ..
        }): State<AppState>,
        Path((obj_id, org_id)): Path<(String, String)>,
    ) -> WebResult {
        obj_service
            .remove_from_org(obj_id.clone(), org_id.clone())
            .await?;

        //let obj = obj_service.get_obj_by_id(obj_id).await?;

        // let users = user_service
        //     .get_users(
        //         vec![user::department_id::equals(Some(department_id))],
        //         0,
        //         100,
        //     )
        //     .await?;
        //
        // let message = format!(r#"You are removed from objective {} "#, obj.name);
        //
        // for user in users {
        //     notification_service
        //         .create_noti(user.pk_user_id, message.clone(), vec![])
        //         .await?;
        // }

        Ok(WebResponse::created(
            "Remove objective from organize sucessfully",
            (),
        ))
    }
    Router::new().route(
        "/:obj_id/remove_from_org/:org_id",
        post(remove_from_org_handler),
    )
}
