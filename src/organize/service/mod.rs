use crate::{
    error::ErrorResponse,
    helpers::id::generate_id,
    prisma::{
        organize::{SetParam, WhereParam},
        user,
    },
};
use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::{offset, DateTime};
use prisma_client_rust::query_core::schema_builder::constants::filters;

use crate::prisma::{organize, PrismaClient};

use super::model::response::{organize_select, OrganizeResponse, OrganizeSelect};

#[derive(Clone)]
pub struct OrganizeService {
    pub db: Arc<PrismaClient>,
    salt: SaltString,
}

impl OrganizeService {
    pub fn init(db: &Arc<PrismaClient>) -> Self {
        Self {
            db: db.clone(),
            salt: SaltString::generate(&mut OsRng),
        }
    }

    pub async fn get_organizes(
        &self,
        filters: Vec<WhereParam>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<OrganizeSelect>, ErrorResponse> {
        let orgs = self
            .db
            .organize()
            .find_many(filters)
            .skip(offset)
            .take(limit)
            .select(organize_select::select())
            .exec()
            .await?;

        Ok(orgs)
    }

    pub async fn get_organize_by_id(
        &self,
        org_id: String,
    ) -> Result<OrganizeSelect, ErrorResponse> {
        let org = self
            .db
            .organize()
            .find_unique(organize::pk_organize_id::equals(org_id))
            .select(organize_select::select())
            .exec()
            .await?
            .ok_or_else(|| ErrorResponse::NotFound)?;

        Ok(org)
    }

    pub async fn create_organize(
        &self,
        name: String,
        address: String,
        contact: String,
        owner_id: String,
        params: Vec<SetParam>,
    ) -> Result<OrganizeSelect, ErrorResponse> {
        self.db
            .organize()
            .create(
                generate_id(),
                name,
                address,
                contact,
                user::pk_user_id::equals(owner_id),
                params,
            )
            .select(organize_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn update_organize(
        &self,
        org_id: String,
        changes: Vec<SetParam>,
    ) -> Result<OrganizeSelect, ErrorResponse> {
        self.db
            .organize()
            .update(organize::pk_organize_id::equals(org_id), changes)
            .select(organize_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn delete_organize(&self, org_id: String) -> Result<OrganizeSelect, ErrorResponse> {
        let deleted_org = self
            .db
            .organize()
            .delete(organize::pk_organize_id::equals(org_id))
            .select(organize_select::select())
            .exec()
            .await?;

        Ok(deleted_org)
    }
}
