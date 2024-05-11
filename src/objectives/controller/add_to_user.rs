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
  path = "/api/v1/objective/{obj_id}/add_to_user/{user_id}",
  params(
    ("obj_id" = String, Path, description = "Objective ID"),
    ("user_id" = String, Path, description = "User ID")
  ),
  responses(
    (
      status = 201,
      description = "Add objective to user successfully",
      body = WebResponse,
      example = json! (
        {
          "code": 201,
          "message": "Add objective to user successfully",
          "data": null,
          "error": ""
        }
      )
    ),
  )
)]
pub fn add_to_user() -> Router<AppState> {
    async fn add_to_user_handler(
        State(AppState {
            notification_service,
            obj_service,
            ..
        }): State<AppState>,
        Path((obj_id, user_id)): Path<(String, String)>,
    ) -> WebResult {
        obj_service
            .add_to_user(obj_id.clone(), user_id.clone())
            .await?;

        let obj = obj_service.get_obj_by_id(obj_id).await?;

        let message = format!(r#"Mục tiêu mới {} được gán cho bạn"#, obj.name);
        notification_service
            .create_noti(user_id, message.clone(), vec![])
            .await?;

        Ok(WebResponse::created(
            "Add objective to user sucessfully",
            (),
        ))
    }
    Router::new().route("/:obj_id/add_to_user/:user_id", post(add_to_user_handler))
}

#[utoipa::path(
  post,
  tag = "Objective",
  path = "/api/v1/objective/{obj_id}/remove_from_user/{user_id}",
  params(
    ("obj_id" = String, Path, description = "Objective ID"),
    ("user_id" = String, Path, description = "User ID")
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
pub fn remove_from_user() -> Router<AppState> {
    async fn remove_from_user_handler(
        State(AppState {
            obj_service,
            user_service,
            notification_service,
            ..
        }): State<AppState>,
        Path((obj_id, user_id)): Path<(String, String)>,
    ) -> WebResult {
        obj_service
            .remove_from_user(obj_id.clone(), user_id.clone())
            .await?;

        let obj = obj_service.get_obj_by_id(obj_id).await?;

        let user = user_service.get_user_by_id(user_id.clone()).await?;

        let message = format!(r#"Bạn đã rời khỏi mục tiêu {}  "#, obj.name.clone());
        notification_service
            .create_noti(user.pk_user_id.clone(), message.clone(), vec![])
            .await?;

        // let users = user_service
        //     .get_users(vec![user::organize::equals(Some(org))], 0, 100)
        //     .await?;
        //
        // let message = format!(r#"You are removed from objective {} "#, obj.name);
        //
        // for user in users {
        //     notification_service
        //         .create_noti(user.pk_user_id, message.clone(), vec![])
        //         .await?;
        // }

        Ok(WebResponse::created("Remove obj from user sucessfully", ()))
    }
    Router::new().route(
        "/:obj_id/remove_from_user/:user_id",
        post(remove_from_user_handler),
    )
}
