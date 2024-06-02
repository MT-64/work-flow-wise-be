use axum::{
    extract::{Path, State},
    routing::put,
    Router,
};
use chrono::DateTime;
use prisma_client_rust::PrismaValue;
use validator::HasLen;

use crate::{
    error::ErrorResponse,
    helpers::validation::validation_message,
    key_result::model::{
        request::{GradingKr, UpdateKrProgressRequest, UpdateKrRequest},
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
            notification_service,
            keyresult_service,
            ..
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
        let message = format!(
            r#"Kết quả then chốt {} có sự thay đổi"#,
            updated_kr.name.clone()
        );
        notification_service
            .create_noti(updated_kr.name.clone(), message.clone(), vec![])
            .await?;

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
            notification_service,
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
                vec![
                    key_result::objective_id::equals(obj.pk_objective_id.clone()),
                    key_result::status::equals(true),
                ],
                0,
                500,
            )
            .await?;
        /// calulation progress for obj where parent of kr
        let mut progress_obj = 0.0;
        let mut num_kr = 0.0;
        //       let mut weight = 0.0;
        for kr in &krs {
            //           progress_obj += kr.progress * kr.target;
            progress_obj += kr.progress;
            num_kr += 1.0;
            //         weight += kr.target;
        }
        if num_kr == 0.0 {
            num_kr = 1.0;
        }

        let _ = obj_service
            .update_obj(
                obj.pk_objective_id,
                vec![objective::progress::set(Some(
                    (progress_obj / num_kr) as f64,
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
                let mut num_obj = 0.0;
                //let mut weigth_department = 0.0;

                for obj_user in &all_user_obj {
                    // progress_department_obj += obj_user.progress.unwrap_or(0.0) * obj_user.target;
                    progress_department_obj += obj_user.progress.unwrap_or(0.0);
                    num_obj += 1.0;
                    // weigth_department += obj_user.target;
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
                let department_obj = obj_service.get_obj_by_id(parent_id).await?;
                // update obj organization
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
                        //let mut weigth_org = 0.0;
                        let mut num_department_obj = 0.0;

                        for obj_department in &all_department_obj {
                            // progress_org_obj +=
                            //     obj_department.progress.unwrap_or(0.0) * obj_department.target;
                            // weigth_org += obj_department.target;
                            progress_org_obj += obj_department.progress.unwrap_or(0.0);
                            num_department_obj += 1.0;
                        }

                        if num_department_obj == 0.0 {
                            num_department_obj = 1.0;
                        }
                        let _ = obj_service
                            .update_obj(
                                parent_department_obj_id,
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
        let message = format!(
            r#"Kết quả then chốt {} đã được chấm điểm"#,
            updated_kr.name.clone()
        );
        notification_service
            .create_noti(updated_kr.name.clone(), message.clone(), vec![])
            .await?;

        Ok(WebResponse::ok("Update keyresult successfully", updated_kr))
    }
    Router::new().route("/grading_kr/:kr_id", put(grading_kr_handler))
}

#[utoipa::path(
  put,
  tag = "Key Result",
  path = "/api/v1/kr/update_progress/{kr_id}",
  params(
    ("kr_id" = String, Path, description = "Keyresult ID")
  ),

  request_body(
    content = UpdateKrProgressRequest,
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
pub fn update_kr_progress() -> Router<AppState> {
    async fn update_kr_progress_handler(
        State(AppState {
            notification_service,
            keyresult_service,
            ..
        }): State<AppState>,
        Path(kr_id): Path<String>,
        LoggedInUser(user): LoggedInUser,
        UpdateKrProgressRequest { progress }: UpdateKrProgressRequest,
    ) -> WebResult {
        let mut changes = vec![];

        if let Some(progress) = progress {
            changes.push(key_result::progress::set(progress));
        }

        changes.push(key_result::status::set(false));

        changes.push(key_result::supervisor_grade::set(0.0));

        let updated_kr: KeyResultResponse =
            keyresult_service.update_kr(kr_id, changes).await?.into();

        let message = format!(
            r#"Kết quả then chốt {} có sự thay đổi"#,
            updated_kr.name.clone()
        );
        notification_service
            .create_noti(updated_kr.name.clone(), message.clone(), vec![])
            .await?;

        Ok(WebResponse::ok("Update keyresult successfully", updated_kr))
    }
    Router::new().route("/update_progress/:kr_id", put(update_kr_progress_handler))
}
