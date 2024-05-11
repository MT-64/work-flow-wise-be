use crate::{
    error::ErrorResponse,
    helpers::validation::validation_message,
    key_result::model::{
        request::{GradingKr, UpdateKrRequest},
        response::KeyResultResponse,
    },
    prisma::{
        key_result::{self, deadline},
        objective,
    },
    response::WebResponse,
    state::AppState,
    users::model::loggedin::LoggedInUser,
    WebResult,
};

use axum::{
    extract::{Path, State},
    routing::delete,
    Router,
};

#[utoipa::path(
  delete,
  tag = "Key Result",
  path = "/api/v1/kr/delete/{kr_id}",
  params(
    ("kr_id" = String, Path, description = "Keyresult ID")
  ),
  responses(
    (
      status = 200,
      description = "Deleted keyresult successfully",
      body = WebResponse,
      example = json!(
        {
          "code": 200,
          "message": "Deleted keyresult successfully",
          "data": null,
          "error": ""
        }
      )
    )
  )
)]
pub fn delete_kr() -> Router<AppState> {
    async fn delete_kr_handler(
        State(AppState {
            obj_service,
            notification_service,
            keyresult_service,
            ..
        }): State<AppState>,
        LoggedInUser(_): LoggedInUser,
        Path(kr_id): Path<String>,
    ) -> WebResult {
        ////recalculation
        let kr = keyresult_service.get_kr_by_id(kr_id.clone()).await?;
        let message = format!(r#"Kết quả then chốt {} đã được xóa"#, kr.name.clone());
        notification_service
            .create_noti(kr.user_id.clone(), message.clone(), vec![])
            .await?;

        let obj = obj_service.get_obj_by_id(kr.objective_id).await?;
        /// update obj target        
        let _ = obj_service
            .update_obj(
                obj.pk_objective_id.clone(),
                vec![objective::target::set(
                    obj.target.clone() - kr.target.clone(),
                )],
            )
            .await?;

        //delete kr
        keyresult_service.delete_kr(kr_id).await?;
        ////recalculation
        let krs = keyresult_service
            .get_krs(
                vec![key_result::objective_id::equals(
                    obj.pk_objective_id.clone(),
                )],
                0,
                500,
            )
            .await?;
        let mut progress_obj = 0.0;
        let mut weight = 0.0;
        for kr in &krs {
            progress_obj += kr.progress * kr.target;
            weight += kr.target;
        }

        let _ = obj_service
            .update_obj(
                obj.pk_objective_id,
                vec![objective::progress::set(Some(
                    (progress_obj / weight) as f64,
                ))],
            )
            .await?;
        // update obj department
        match obj.parent_objective_id {
            Some(parent_id) => {
                let all_user_obj = obj_service
                    .get_objs(
                        vec![objective::parent_objective_id::equals(Some(
                            parent_id.clone(),
                        ))],
                        0,
                        500,
                    )
                    .await?;
                let mut progress_department_obj = 0.0;
                let mut weigth_department = 0.0;

                for obj_user in &all_user_obj {
                    progress_department_obj += obj_user.progress.unwrap_or(0.0) * obj_user.target;
                    weigth_department += obj_user.target;
                }

                let _ = obj_service
                    .update_obj(
                        parent_id.clone(),
                        vec![objective::progress::set(Some(
                            (progress_department_obj / weigth_department) as f64,
                        ))],
                    )
                    .await?;
                let department_obj = obj_service.get_obj_by_id(parent_id).await?;
                match department_obj.parent_objective_id {
                    Some(parent_department_obj_id) => {
                        let all_department_obj = obj_service
                            .get_objs(
                                vec![objective::parent_objective_id::equals(Some(
                                    parent_department_obj_id.clone(),
                                ))],
                                0,
                                500,
                            )
                            .await?;
                        let mut progress_org_obj = 0.0;
                        let mut weigth_org = 0.0;
                        for obj_department in &all_department_obj {
                            progress_org_obj +=
                                obj_department.progress.unwrap_or(0.0) * obj_department.target;
                            weigth_org += obj_department.target;
                        }

                        let _ = obj_service
                            .update_obj(
                                parent_department_obj_id,
                                vec![objective::progress::set(Some(
                                    (progress_org_obj / weigth_org) as f64,
                                ))],
                            )
                            .await?;
                    }
                    None => {}
                }
            }
            None => {}
        }
        Ok(WebResponse::ok("Deleted keyresult successfully", ()))
    }
    Router::new().route("/delete/:kr_id", delete(delete_kr_handler))
}
