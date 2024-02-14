use axum::{extract::State, routing::post, Router};

use crate::{
    key_result::model::{request::CreateKrRequest, response::KeyResultResponse},
    prisma::{self, key_result},
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
            keyresult_service, ..
        }): State<AppState>,
        CreateKrRequest {
            name,
            description,
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

        let new_kr: KeyResultResponse = keyresult_service
            .create_kr(
                name,
                user_id,
                objective_id,
                target,
                description,
                deadline,
                metric,
                params,
            )
            .await?
            .into();

        Ok(WebResponse::created(
            "Created key result sucessfully",
            KeyResultResponse::from(new_kr),
        ))
    }
    Router::new().route("/create", post(create_kr_handler))
}
