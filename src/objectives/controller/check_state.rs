use crate::{
    extractors::param::ParamId,
    objectives::model::{request::CheckStateObjRequest, response::ObjectiveResponse},
    prisma::{key_result, objective},
    response::WebResponse,
    state::AppState,
    WebResult,
};

use axum::{
    extract::{Path, State},
    routing::post,
    Router,
};
use chrono::{DateTime, Utc};
#[utoipa::path(
  post,
  tag = "Objective",
  path = "/api/v1/objective/check_state",
  request_body(
    content = CheckStateObjRequest,
    description = "Create Organize Request",
  ),
  responses(
    (
      status = 201,
      description = "Check state objective by id",
      body = ObjectiveResponse,
      example = json! (
        {
          "code": 200,
          "message": "Check state objective by id successfully",
          "data": {
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn check_state_obj() -> Router<AppState> {
    async fn check_state_obj_handler(
        State(AppState {
            obj_service,
            keyresult_service,
            ..
        }): State<AppState>,
        CheckStateObjRequest { list_obj }: CheckStateObjRequest,
    ) -> WebResult {
        let mut objs = vec![];
        for obj_id in &list_obj {
            let obj = obj_service.get_obj_by_id(obj_id.clone()).await?;
            let krs = keyresult_service
                .get_krs(
                    vec![key_result::objective_id::equals(obj_id.clone())],
                    0,
                    500,
                )
                .await?;
            let mut check_state = true;

            for kr in &krs {
                if kr.status == false {
                    check_state = false;
                    break;
                }
            }

            if obj.deadline.with_timezone(&Utc).timestamp_millis() > Utc::now().timestamp_millis() {
                let obj = obj_service
                    .update_obj(
                        obj_id.to_string(),
                        vec![
                            objective::status::set(true),
                            objective::achievement::set(Some(
                                crate::prisma::Achievement::NonAchievement,
                            )),
                        ],
                    )
                    .await?;
            }

            if obj.deadline.with_timezone(&Utc).timestamp_millis() < Utc::now().timestamp_millis()
                || check_state == true
            {
                if obj.progress.unwrap_or(0.0) == 100.0 {
                    let obj = obj_service
                        .update_obj(
                            obj_id.to_string(),
                            vec![
                                objective::status::set(true),
                                objective::achievement::set(Some(
                                    crate::prisma::Achievement::Achievement,
                                )),
                            ],
                        )
                        .await?;
                    objs.push(obj);
                }

                objs.push(obj);
            }
        }
        Ok(WebResponse::ok("Get state objective successfully", objs))
    }
    Router::new().route("/check_state", post(check_state_obj_handler))
}
