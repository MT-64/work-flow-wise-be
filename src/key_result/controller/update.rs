use axum::{
    extract::{Path, State},
    routing::put,
    Router,
};
use chrono::DateTime;
use prisma_client_rust::PrismaValue;

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

#[utoipa::path(
  put,
  tag = "Key Result",
  path = "/api/v1/kr/update/{kr_id}",
  params(
    ("kr_id" = String, Path, description = "Keyresult ID")
  ),

  request_body(
    content = UpdateKrRequest,
    description = "Update keyresult request",
  ),
  responses(
    (
      status = 200,
      description = "Updated keyresult successfully",
      body = KeyResultResponse,
      example = json!(
        {
          "code": 200,
          "message": "Updated keyresult successfully",
          "data": {
                     },
          "error": ""
        }
      )
    )
  )
)]
pub fn update_kr() -> Router<AppState> {
    async fn update_kr_handler(
        State(AppState {
            keyresult_service, ..
        }): State<AppState>,
        Path(kr_id): Path<String>,
        LoggedInUser(user): LoggedInUser,
        UpdateKrRequest {
            name,
            user_id,
            objective_id,
            description,
            target,
            progress,
            deadline,
        }: UpdateKrRequest,
    ) -> WebResult {
        let mut changes = vec![];

        if let Some(name) = name {
            changes.push(key_result::name::set(name));
        }

        if let Some(description) = description {
            changes.push(key_result::description::set(description));
        }

        if let Some(target) = target {
            changes.push(key_result::target::set(target))
        }

        if let Some(progress) = progress {
            changes.push(key_result::progress::set(progress));
        }

        if let Some(deadline) = deadline {
            changes.push(key_result::deadline::set(
                DateTime::from_timestamp(deadline, 0)
                    .unwrap()
                    .fixed_offset(),
            ))
        }

        let updated_kr: KeyResultResponse =
            keyresult_service.update_kr(kr_id, changes).await?.into();
        Ok(WebResponse::ok("Update keyresult successfully", updated_kr))
    }
    Router::new().route("/update/:kr_id", put(update_kr_handler))
}

#[utoipa::path(
  put,
  tag = "Key Result",
  path = "/api/v1/kr/grading_kr/{kr_id}",
  params(
    ("kr_id" = String, Path, description = "Keyresult ID")
  ),

  request_body(
    content = GradingKr,
    description = "Update keyresult request",
  ),
  responses(
    (
      status = 200,
      description = "Updated keyresult successfully",
      body = KeyResultResponse,
      example = json!(
        {
          "code": 200,
          "message": "Updated keyresult successfully",
          "data": {
                     },
          "error": ""
        }
      )
    )
  )
)]
pub fn grading_kr() -> Router<AppState> {
    async fn grading_kr_handler(
        State(AppState {
            obj_service,
            keyresult_service,
            ..
        }): State<AppState>,
        Path(kr_id): Path<String>,
        LoggedInUser(user): LoggedInUser,
        GradingKr { grade }: GradingKr,
    ) -> WebResult {
        let mut changes = vec![];

        let kr = keyresult_service.get_kr_by_id(kr_id.clone()).await?;

        let obj = obj_service.get_obj_by_id(kr.objective_id).await?;

        if obj.supervisor_id != user.pk_user_id {
            return Err(ErrorResponse::Permissions);
        }

        changes.push(key_result::supervisor_grade::set(grade.clone()));
        changes.push(key_result::status::set(true));

        let updated_kr: KeyResultResponse =
            keyresult_service.update_kr(kr_id, changes).await?.into();

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
            progress_obj += kr.supervisor_grade * kr.target;
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

        Ok(WebResponse::ok("Update keyresult successfully", updated_kr))
    }
    Router::new().route("/grading_kr/:kr_id", put(grading_kr_handler))
}
