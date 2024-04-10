use crate::{prisma::{self, period::{self}}, periods::model::{request::PeriodQueryRequest, response::PeriodResponse}};
use axum::{extract::{State, Path}, routing::get, Router};
use chrono::DateTime;
use prisma_client_rust::query_core::schema_builder::constants::filters;

use crate::{
    extractors::param::ParamId,
    response::WebResponse,
    state::AppState,
    WebResult,
};

#[utoipa::path(
  get,
  tag = "Period",
  path = "/api/v1/period",
  params(
    ("offset" = inline(Option<i64>), Query, description = "Starting point"),
    ("limit" = inline(Option<i32>), Query, description = "Limit"),
    ("id" = inline(Option<String>), Query, description = "Period id"),
    ("orgId" = inline(Option<String>), Query, description = "Organize id"),
    ("name" = inline(Option<String>), Query, description = "Period name"),
    ("startDate" = inline(Option<i64>), Query, description = "Start time "),
    ("endDate" = inline(Option<i64>), Query, description = "End time "),
  ),
  responses(
    (
      status = 200,
      description = "Get periods",
      body = Vec<PeriodResponse>,
      example = json!(
        {
          "code": 200,
          "message": "Get all periods successfully",
          "data": [
            
          ],
          "error": ""
        }
      )
    ),
  )
)]
pub fn get_periods() -> Router<AppState> {
    async fn get_periods_handler(
        State(AppState { period_service, .. }): State<AppState>,
        PeriodQueryRequest { offset, limit, id, name, organize_id, start_date, end_date }:PeriodQueryRequest

    ) -> WebResult {
        let offset = offset.unwrap_or(0);

        let limit = match limit {
            Some(limit) => match limit {
                0..=50 => limit,
                _ => 10,
            },
            None => 10,
        };

        let mut filters = vec![];

        if let Some(id) = id {
            filters.push(period::pk_period_id::equals(id));
        }

        if let Some(organize_id) = organize_id {
            filters.push(period::organize_id::equals(organize_id));
        }

        if let Some(name) = name {
            filters.push(period::name::equals(name));
        }

        if let Some(start_date) = start_date {
            filters.push(period::start_date::gt(DateTime::from_timestamp(start_date, 0).unwrap().fixed_offset()))
        }

        if let Some(end_date) = end_date {
            filters.push(period::end_date::lt(DateTime::from_timestamp(end_date, 0).unwrap().fixed_offset()))
        }


        let periods: Vec<PeriodResponse> = period_service
            .get_periods(filters, offset, limit)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();
        Ok(WebResponse::ok("Get periods successfully", periods))
    }
    Router::new().route("/", get(get_periods_handler))
}

#[utoipa::path(
  get,
  tag = "Period",
  path = "/api/v1/period/{period_id}",
  params(
    ("period_id" = String, Path, description = "Period ID")
  ),
  responses(
    (
      status = 201,
      description = "Get period by period id",
      body = PeriodResponse,
      example = json! (
        {
          "code": 200,
          "message": "Get period by id successfully",
          "data": {
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn get_period() -> Router<AppState> {
    async fn get_period_handler(
        State(AppState { period_service, .. }): State<AppState>,
        Path(period_id): Path<String>,
    ) -> WebResult {
        let period: PeriodResponse =  period_service.get_period_by_id(period_id).await?.into();
        Ok(WebResponse::ok("Get period by id successfully", period))
    }
    Router::new().route("/:period_id", get(get_period_handler))
}
