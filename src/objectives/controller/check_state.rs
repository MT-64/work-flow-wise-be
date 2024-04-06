use crate::{
    extractors::param::ParamId,
    objectives::model::response::ObjectiveResponse,
    prisma::{key_result, objective},
    response::WebResponse,
    state::AppState,
    WebResult,
};

use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use chrono::{DateTime, Utc};
#[utoipa::path(
  get,
  tag = "Objective",
  path = "/api/v1/objective/check_state/(obj_id}",
  params(
    ("obj_id" = String, Path, description = "obj ID")
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
        Path(obj_id): Path<String>,
    ) -> WebResult {
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

        if obj.deadline.with_timezone(&Utc).timestamp_millis() < Utc::now().timestamp_millis()
            || check_state == true
        {
            let obj = obj_service
                .update_obj(obj_id, vec![objective::status::set(true)])
                .await?;

            return Ok(WebResponse::ok(
                "Get objective by parent id successfully",
                obj,
            ));
        }

        Ok(WebResponse::ok(
            "Get objective by parent id successfully",
            obj,
        ))
    }
    Router::new().route("/check_state/:obj_id", get(check_state_obj_handler))
}
