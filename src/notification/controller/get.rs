use crate::{notification::model::request::NotificationQueryRequest, prisma::notification};
use axum::{extract::{State, Path}, routing::get, Router};
use chrono::DateTime;
use prisma_client_rust::query_core::schema_builder::constants::filters;

use crate::{
    extractors::param::ParamId,
    notification::model::response::NotificationResponse,
    response::WebResponse,
    state::AppState,
    WebResult,
};

#[utoipa::path(
  get,
  tag = "Notification",
  path = "/api/v1/notification",
  params(
    ("offset" = inline(Option<i64>), Query, description = "Starting point"),
    ("limit" = inline(Option<i32>), Query, description = "Limit"),
    ("userId" = inline(Option<String>), Query, description = "User id"),
    ("timestamp" = inline(Option<i64>), Query, description = "Timestamp"),
    ("status" = inline(Option<bool>), Query, description = "is read"),
  ),
  responses(
    (
      status = 200,
      description = "Get noties",
      body = Vec<NotificationResponse>,
      example = json!(
        {
          "code": 200,
          "message": "Get all notification successfully",
          "data": [
            
          ],
          "error": ""
        }
      )
    ),
  )
)]
pub fn get_noties() -> Router<AppState> {
    async fn get_noties_handler(
        State(AppState { notification_service, .. }): State<AppState>,
        NotificationQueryRequest { offset, limit, user_id, status, timestamp }: NotificationQueryRequest
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

         if let Some(user_id) = user_id {
            filters.push(notification::user_id::equals(user_id));
        }
       

        if let Some(status) = status {
            filters.push(notification::status::equals(status));
        }


        if let Some(timestamp) = timestamp {
            filters.push(notification::timestamp::gte(
                DateTime::from_timestamp(timestamp, 0)
                    .unwrap()
                    .fixed_offset(),
            ));
        }

        let noties: Vec<NotificationResponse> = notification_service
            .get_noties(filters, offset, limit)
            .await?
            .into_iter()
            .map(|n| n.into())
            .collect();
        Ok(WebResponse::ok("Get noties successfully", noties))
    }
    Router::new().route("/", get(get_noties_handler))
}
