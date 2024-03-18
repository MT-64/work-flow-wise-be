use crate::prisma::{objective, objective_on_department, objective_on_org, objective_on_user};
use chrono::Utc;
use is_empty::IsEmpty;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

objective::select!(objective_select {
    pk_objective_id
    period_id
    supervisor_id
    obj_type
    name
    description
    target
    progress
    status
    created_at
    updated_at
    deadline
});

objective_on_department::select!(obj_id_on_department_select { obj_id });
objective_on_org::select!(obj_id_on_org { obj_id });
objective_on_user::select!(obj_id_on_user { obj_id });

pub type ObjSelect = objective_select::Data;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveResponse {
    pub obj_id: String,
    pub period_id: String,
    pub supervisor_id: String,
    pub obj_type: crate::prisma::ObjectiveType,
    pub name: String,
    pub description: Option<String>,
    pub target: String,
    pub progress: Option<f64>,
    pub status: bool,
    pub deadline: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<ObjSelect> for ObjectiveResponse {
    fn from(
        ObjSelect {
            pk_objective_id,
            period_id,
            obj_type,
            name,
            supervisor_id,
            description,
            target,
            progress,
            status,
            deadline,
            created_at,
            updated_at,
        }: ObjSelect,
    ) -> Self {
        Self {
            obj_id: pk_objective_id,
            period_id,
            supervisor_id,
            obj_type,
            name,
            description,
            target,
            progress,
            status,
            deadline: deadline.with_timezone(&Utc).timestamp(),
            created_at: created_at.with_timezone(&Utc).timestamp(),
            updated_at: updated_at.with_timezone(&Utc).timestamp(),
        }
    }
}
