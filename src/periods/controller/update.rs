use axum::{
    extract::{Path, State},
    routing::put,
    Router,
};
use chrono::DateTime;

use crate::{
    helpers::validation::validation_message,
    periods::model::{request::UpdatePeriodRequest, response::PeriodResponse},
    prisma::period,
    response::WebResponse,
    state::AppState,
    users::model::loggedin::LoggedInUser,
    WebResult,
};

#[utoipa::path(
  put,
  tag = "Period",
  path = "/api/v1/period/update/{period_id}",
  params(
    ("period_id" = String, Path, description = "Period ID")
  ),

  request_body(
    content = UpdatePeriodRequest,
  content_type = "multipart/form-data",
    description = "Update period request",
  ),
  responses(
    (
      status = 200,
      description = "Updated period successfully",
      body = PeriodResponse,
      example = json!(
        {
          "code": 200,
          "message": "Updated period successfully",
          "data": {
                     },
          "error": ""
        }
      )
    )
  )
)]
pub fn update_period() -> Router<AppState> {
    async fn update_period_handler(
        State(AppState { period_service, .. }): State<AppState>,
        Path(period_id): Path<String>,
        LoggedInUser(user): LoggedInUser,
        UpdatePeriodRequest {
            name,
            start_date,
            end_date,
        }: UpdatePeriodRequest,
    ) -> WebResult {
        let mut changes = vec![];

        if let Some(name) = name {
            changes.push(period::name::set(name));
        }

        if let Some(start_date) = start_date {
            changes.push(period::start_date::set(
                DateTime::from_timestamp(start_date, 0)
                    .unwrap()
                    .fixed_offset(),
            ))
        }

        if let Some(end_date) = end_date {
            changes.push(period::end_date::set(
                DateTime::from_timestamp(end_date, 0)
                    .unwrap()
                    .fixed_offset(),
            ))
        }

        let updated_period: PeriodResponse = period_service
            .update_period(period_id, changes)
            .await?
            .into();
        Ok(WebResponse::ok(
            "Update period successfully",
            updated_period,
        ))
    }
    Router::new().route("/update/:period_id", put(update_period_handler))
}
