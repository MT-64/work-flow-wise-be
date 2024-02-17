use axum::{extract::State, routing::post, Router};

use crate::{
    periods::model::{request::CreatePeriodRequest, response::PeriodResponse},
    prisma::{self, period},
    response::WebResponse,
    state::AppState,
    WebResult,
};

#[utoipa::path(
  post,
  tag = "Period",
  path = "/api/v1/period/create",
  request_body(
    content = CreatePeriodRequest,
    description = "Create Period Request",
  ),
  responses(
    (
      status = 201,
      description = "Period created",
      body = PeriodResponse,
      example = json! (
        {
          "code": 201,
          "message": "Created new period successfully",
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
pub fn create_period() -> Router<AppState> {
    async fn create_period_handler(
        State(AppState { period_service, .. }): State<AppState>,
        CreatePeriodRequest {
            name,
            organize_id,
            start_date,
            end_date,
        }: CreatePeriodRequest,
    ) -> WebResult {
        let mut params = vec![];

        let new_period: PeriodResponse = period_service
            .create_period(name, organize_id, start_date, end_date, params)
            .await?
            .into();

        Ok(WebResponse::created(
            "Created period sucessfully",
            PeriodResponse::from(new_period),
        ))
    }
    Router::new().route("/create", post(create_period_handler))
}
