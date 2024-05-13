use crate::prisma::objective;
use crate::prisma::user::{self, objective_on_user};
use crate::users::model::loggedin::LoggedInUser;
use axum::{
    extract::{Path, State},
    routing::delete,
    Router,
};

use crate::{
    helpers::validation::validation_message, response::WebResponse, state::AppState, WebResult,
};

#[utoipa::path(
  delete,
  tag = "Objective",
  path = "/api/v1/objective/delete/{obj_id}",
  params(
    ("obj_id" = String, Path, description = "Objective ID")
  ),
  responses(
    (
      status = 200,
      description = "Deleted objective successfully",
      body = WebResponse,
      example = json!(
        {
          "code": 200,
          "message": "Deleted objective successfully",
          "data": null,
          "error": ""
        }
      )
    )
  )
)]
pub fn delete_obj() -> Router<AppState> {
    async fn delete_obj_handler(
        State(AppState {
            user_service,
            obj_service,
            notification_service,
            ..
        }): State<AppState>,
        LoggedInUser(_): LoggedInUser,
        Path(obj_id): Path<String>,
    ) -> WebResult {
        let deleted_obj = obj_service.delete_obj(obj_id).await?;
        let users = user_service
            .get_users_by_obj(deleted_obj.pk_objective_id.clone())
            .await?;
        let message = format!(r#"Mục tiêu {} đã được xóa"#, deleted_obj.name.clone());
        for user in users.iter() {
            notification_service
                .create_noti(user.pk_user_id.clone(), message.clone(), vec![])
                .await?;
        }

        match deleted_obj.parent_objective_id {
            Some(ref parent_id) => {
                let department_obj = obj_service.get_obj_by_id(parent_id.clone()).await?;

                let all_user_obj = obj_service
                    .get_objs(
                        vec![objective::parent_objective_id::equals(Some(
                            parent_id.clone(),
                        ))],
                        0,
                        100,
                    )
                    .await?;
                let mut progress_department_obj = 0.0;
                let mut num_obj = 0.0;

                for obj_user in &all_user_obj {
                    progress_department_obj += obj_user.progress.unwrap_or(0.0);
                    num_obj += 1.0;
                }
                if num_obj == 0.0 {
                    num_obj = 1.0;
                }

                let _ = obj_service
                    .update_obj(
                        parent_id.clone(),
                        vec![objective::progress::set(Some(
                            (progress_department_obj / num_obj) as f64,
                        ))],
                    )
                    .await?;
                //// update organize objective
                match department_obj.parent_objective_id {
                    Some(parent_department_obj_id) => {
                        let all_department_obj = obj_service
                            .get_objs(
                                vec![objective::parent_objective_id::equals(Some(
                                    parent_department_obj_id.clone(),
                                ))],
                                0,
                                100,
                            )
                            .await?;
                        let mut progress_org_obj = 0.0;
                        let mut num_department_obj = 0.0;

                        for obj_department in &all_department_obj {
                            progress_org_obj += obj_department.progress.unwrap_or(0.0);
                            num_department_obj += 1.0;
                        }

                        if num_department_obj == 0.0 {
                            num_department_obj = 1.0;
                        }
                        let _ = obj_service
                            .update_obj(
                                parent_department_obj_id.clone(),
                                vec![objective::progress::set(Some(
                                    (progress_org_obj / num_department_obj) as f64,
                                ))],
                            )
                            .await?;
                    }
                    None => {}
                }
            }
            None => {}
        }

        Ok(WebResponse::ok("Deleted objective successfully", ()))
    }
    Router::new().route("/delete/:obj_id", delete(delete_obj_handler))
}
