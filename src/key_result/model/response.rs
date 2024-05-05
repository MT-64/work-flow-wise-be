use chrono::Utc;
use is_empty::IsEmpty;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::{helpers::aws_path::get_aws_path, prisma::key_result};

key_result::select!(keyresult_select {
    pk_kr_id
    objective_id
    name
    description
    user_id
    target
    metric
    progress
    status
    deadline
    created_at
    updated_at
    supervisor_grade
    file_shared
});

pub type KrSelect = keyresult_select::Data;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct KeyResultResponse {
    pub keyresult_id: String,
    pub objective_id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub metric: String,
    pub target: f64,
    pub progress: f64,
    pub status: bool,
    pub deadline: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub supervisor_grade: f64,
    pub file_shared: Vec<String>,
}

impl From<KrSelect> for KeyResultResponse {
    fn from(
        KrSelect {
            supervisor_grade,
            pk_kr_id,
            objective_id,
            name,
            description,
            user_id,
            target,
            metric,
            progress,
            status,
            deadline,
            created_at,
            updated_at,
            file_shared,
        }: KrSelect,
    ) -> Self {
        let file_shared = file_shared.into_iter().map(|f| get_aws_path(&f)).collect();
        Self {
            keyresult_id: pk_kr_id,
            supervisor_grade,
            objective_id,
            user_id,
            name,
            description,
            metric,
            target,
            progress,
            status,
            deadline: deadline.with_timezone(&Utc).timestamp(),
            created_at: created_at.with_timezone(&Utc).timestamp(),
            updated_at: updated_at.with_timezone(&Utc).timestamp(),
            file_shared,
        }
    }
}
