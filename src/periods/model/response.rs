use chrono::Utc;
use is_empty::IsEmpty;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::prisma::period;

period::select!(period_select {
    pk_period_id
    name
    start_date
    end_date
});

pub type PeriodSelect = period_select::Data;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PeriodResponse {
    pub pk_period_id: String,
    pub name: String,
    pub start_date: i64,
    pub end_date: i64,
}

impl From<PeriodSelect> for PeriodResponse {
    fn from(
        PeriodSelect {
            pk_period_id,
            name,
            start_date,
            end_date,
        }: PeriodSelect,
    ) -> Self {
        Self {
            pk_period_id,
            name,
            start_date: start_date.with_timezone(&Utc).timestamp(),
            end_date: end_date.with_timezone(&Utc).timestamp(),
        }
    }
}

