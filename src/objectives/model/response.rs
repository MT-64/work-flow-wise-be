use crate::prisma::{
    objective, objective::WhereParam, objective_on_department, objective_on_org, objective_on_user,
};
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
    parent_objective_id
    status
    created_at
    updated_at
    deadline
    achievement
    metric
    obj_for
    expected
});

objective_on_department::select!(obj_id_on_department_select { obj_id });
objective_on_org::select!(obj_id_on_org { obj_id });
objective_on_user::select!(obj_id_on_user { obj_id });

pub type ObjSelect = objective_select::Data;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub enum Achievement {
    Achievement = 0,
    NonAchievement = 1,
    Exceed = 2,
}
impl From<i32> for Achievement {
    fn from(achievement: i32) -> Self {
        match achievement {
            1 => Self::NonAchievement,
            2 => Self::Exceed,
            _ => Self::Achievement,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub enum ObjectiveMetric {
    Quantity = 0,
    Percent = 1,
    Time = 2,
    Money = 3,
}
impl From<i32> for ObjectiveMetric {
    fn from(metric: i32) -> Self {
        match metric {
            1 => Self::Percent,
            2 => Self::Time,
            3 => Self::Money,
            _ => Self::Quantity,
        }
    }
}
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
    pub target: f64,
    pub progress: Option<f64>,
    pub status: bool,
    pub deadline: i64,
    pub expected: f64,
    pub created_at: i64,
    pub updated_at: i64,
    pub parent_objective_id: String,
    pub achievement: crate::prisma::Achievement,
    pub metric: crate::prisma::ObjectiveMetric,
    pub obj_for: crate::prisma::ObjectiveFor,
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
            expected,
            updated_at,
            parent_objective_id,
            achievement,
            metric,
            obj_for,
        }: ObjSelect,
    ) -> Self {
        Self {
            obj_id: pk_objective_id,
            period_id,
            supervisor_id,
            obj_type,
            obj_for,
            metric,
            expected,
            achievement: achievement.unwrap_or(crate::prisma::Achievement::Other),
            name,
            description,
            target,
            progress,
            status,
            parent_objective_id: parent_objective_id.unwrap_or_default(),
            deadline: deadline.with_timezone(&Utc).timestamp(),
            created_at: created_at.with_timezone(&Utc).timestamp(),
            updated_at: updated_at.with_timezone(&Utc).timestamp(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ProgressResponse {
    pub progress: f64,
}
