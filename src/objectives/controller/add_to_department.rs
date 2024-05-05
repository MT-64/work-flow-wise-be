use axum::{
    extract::{Path, State},
    routing::post,
    Router,
};

use crate::{
    objectives::model::{
        request::{AddMultipleObjToDepartment, CreateObjRequest},
        response::ObjectiveResponse,
    },
    prisma::{self, objective, user},
    response::WebResponse,
    state::AppState,
    WebResult,
};

#[utoipa::path(
  post,
  tag = "Objective",
  path = "/api/v1/objective/{obj_id}/add_to_department/{department_id}",
  params(
    ("obj_id" = String, Path, description = "Objective ID"),
    ("department_id" = String, Path, description = "Department ID")
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
pub fn add_to_department() -> Router<AppState> {
    async fn add_to_department_handler(
        State(AppState {
            obj_service,
            user_service,
            notification_service,
            ..
        }): State<AppState>,
        Path((obj_id, department_id)): Path<(String, String)>,
    ) -> WebResult {
        obj_service
            .add_to_department(obj_id.clone(), department_id.clone())
            .await?;

        let obj = obj_service.get_obj_by_id(obj_id).await?;

        let users = user_service
            .get_users(
                vec![user::department_id::equals(Some(department_id))],
                0,
                100,
            )
            .await?;

        let message = format!(r#"New objective {} is assigned to you"#, obj.name);

        for user in users {
            notification_service
                .create_noti(user.pk_user_id, message.clone(), vec![])
                .await?;
        }

        Ok(WebResponse::created(
            "Add objective to department sucessfully",
            (),
        ))
    }
    Router::new().route(
        "/:obj_id/add_to_department/:department_id",
        post(add_to_department_handler),
    )
}

#[utoipa::path(
  post,
  tag = "Objective",
  path = "/api/v1/objective/{obj_id}/remove_from_department/{department_id}",
  params(
    ("obj_id" = String, Path, description = "Objective ID"),
    ("department_id" = String, Path, description = "Department ID")
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
pub fn remove_from_department() -> Router<AppState> {
    async fn remove_from_department_handler(
        State(AppState {
            obj_service,
            user_service,
            notification_service,
            ..
        }): State<AppState>,
        Path((obj_id, department_id)): Path<(String, String)>,
    ) -> WebResult {
        obj_service
            .remove_from_department(obj_id.clone(), department_id.clone())
            .await?;

        let obj = obj_service.get_obj_by_id(obj_id).await?;

        let users = user_service
            .get_users(
                vec![user::department_id::equals(Some(department_id))],
                0,
                100,
            )
            .await?;

        let message = format!(r#"You are removed from objective {} "#, obj.name);

        for user in users {
            notification_service
                .create_noti(user.pk_user_id, message.clone(), vec![])
                .await?;
        }

        Ok(WebResponse::created(
            "Add objective to department sucessfully",
            (),
        ))
    }
    Router::new().route(
        "/:obj_id/remove_from_department/:department_id",
        post(remove_from_department_handler),
    )
}

#[utoipa::path(
  post,
  tag = "Objective",
  path = "/api/v1/objective/add_multiple_to_department",
  request_body(content = AddMultipleObjToDepartment, description = "Add Multuple Obj to Department"),
  responses(
    (
      status = 201,
      description = "Add multiple objective to department successfully",
      body = WebResponse,
      example = json! (
        {
          "code": 201,
          "message": "Add multiple objective to department successfully",
          "data": null,
          "error": ""
        }
      )
    ),
  )
)]
pub fn add_multiple_to_department() -> Router<AppState> {
    async fn add_multiple_to_department_handler(
        State(AppState {
            obj_service,
            user_service,
            notification_service,
            ..
        }): State<AppState>,
        AddMultipleObjToDepartment {
            list_obj,
            department_id,
        }: AddMultipleObjToDepartment,
    ) -> WebResult {
        for obj_id in list_obj {
            obj_service
                .add_to_department(obj_id.clone(), department_id.clone())
                .await?;

            let obj = obj_service.get_obj_by_id(obj_id).await?;

            let users = user_service
                .get_users(
                    vec![user::department_id::equals(Some(department_id.clone()))],
                    0,
                    100,
                )
                .await?;

            let message = format!(r#"New objective {} is assigned to you"#, obj.name);

            for user in users {
                notification_service
                    .create_noti(user.pk_user_id, message.clone(), vec![])
                    .await?;
            }
        }
        Ok(WebResponse::created(
            "Add objective to department sucessfully",
            (),
        ))
    }
    Router::new().route(
        "/:obj_id/add_to_department/:department_id",
        post(add_multiple_to_department_handler),
    )
}
