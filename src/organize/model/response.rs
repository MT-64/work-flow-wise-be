use crate::prisma::objective_on_org;
use crate::prisma::organize;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

organize::select!(organize_select {
    pk_organize_id
    owner_id
    name
    address
    contact
});

pub type OrganizeSelect = organize_select::Data;

objective_on_org::select!(org_id_on_obj { org_id });
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizeResponse {
    pub id: String,
    pub owner_id: String,
    pub name: String,
    pub address: String,
    pub contact: String,
}

impl From<OrganizeSelect> for OrganizeResponse {
    fn from(
        OrganizeSelect {
            pk_organize_id,
            owner_id,
            name,
            address,
            contact,
        }: OrganizeSelect,
    ) -> Self {
        Self {
            id: pk_organize_id,
            owner_id,
            name,
            address,
            contact,
        }
    }
}
