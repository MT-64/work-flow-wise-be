use crate::prisma::department::{self};
use crate::prisma::objective_on_department;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

department::select!(department_select {
    pk_department_id
    organize_id
    manager_id
    name
});

pub type DepartmentSelect = department_select::Data;

objective_on_department::select!(department_id_on_obj { department_id });

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DepartmentResponse {
    pub id: String,
    pub organize_id: String,
    pub manager_id: String,
    pub name: String,
}

impl From<DepartmentSelect> for DepartmentResponse {
    fn from(
        DepartmentSelect {
            pk_department_id,
            organize_id,
            manager_id,
            name,
        }: DepartmentSelect,
    ) -> Self {
        Self {
            id: pk_department_id,
            organize_id,
            manager_id: manager_id.unwrap_or_default(),
            name,
        }
    }
}
