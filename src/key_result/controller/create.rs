use axum::{extract::State, routing::post, Router};

use crate::{
    key_result::model::{request::CreateKrRequest, response::KeyResultResponse},
    prisma::{self, key_result, objective},
    response::WebResponse,
    state::AppState,
    WebResult,
};

#[utoipa::path(
  post,
  tag = "Key Result",
  path = "/api/v1/kr/create",
  request_body(
    content = CreateKrRequest,
    description = "Create Keyresult Request",
  ),
  responses(
    (
      status = 201,
      description = "KeyResult created",
      body = ObjectiveResponse,
      example = json! (
        {
          "code": 201,
          "message": "Created new key_result successfully",
          "data": {
            "createdAt": 1696932804946_i64,
            "updatedAt": 1696932804946_i64
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn create_kr() -> Router<AppState> {
    async fn create_kr_handler(
        State(AppState {
            obj_service,
            keyresult_service,
            notification_service,
            ..
        }): State<AppState>,
        CreateKrRequest {
            name,
            description,
            expected,
            user_id,
            objective_id,
            target,
            metric,
            progress,
            deadline,
        }: CreateKrRequest,
    ) -> WebResult {
        let mut params = vec![];

        if let Some(progress) = progress {
            params.push(key_result::progress::set(progress));
        }

        let obj = obj_service.get_obj_by_id(objective_id.clone()).await?;
        let _ = obj_service
            .update_obj(
                objective_id.clone(),
                vec![objective::target::set(target.clone() + obj.target)],
            )
            .await?;

        let new_kr: KeyResultResponse = keyresult_service
            .create_kr(
                name.clone(),
                user_id.clone(),
                objective_id,
                target,
                expected,
                description,
                deadline,
                metric,
                params,
            )
            .await?
            .into();

        let krs = keyresult_service
            .get_krs(
                vec![key_result::objective_id::equals(
                    obj.pk_objective_id.clone(),
                )],
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
        let message = format!(r#"Kết quả then chốt {} đã được gán cho bạn"#, name);
        notification_service
            .create_noti(user_id, message.clone(), vec![])
            .await?;

        Ok(WebResponse::created(
            "Created key result sucessfully",
            KeyResultResponse::from(new_kr),
        ))
    }
    Router::new().route("/create", post(create_kr_handler))
}
