use crate::prisma::notification;
use chrono::Utc;
use is_empty::IsEmpty;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

notification::select!(notification_select {
    pk_notification_id
    user_id
    message
    status
    timestamp
});

pub type NotiSelect = notification_select::Data;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct NotificationResponse {
    pub notification_id: String,
    pub user_id: String,
    pub message: String,
    pub status: bool,
    pub timestamp: i64,
}

impl From<NotiSelect> for NotificationResponse {
    fn from(
        NotiSelect {
            pk_notification_id,
            user_id,
            message,
            status,
            timestamp,
        }: NotiSelect,
    ) -> Self {
        Self {
            notification_id: pk_notification_id,
            user_id,
            message,
            status,
            timestamp: timestamp.with_timezone(&Utc).timestamp(),
        }
    }
}
