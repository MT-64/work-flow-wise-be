use crate::{error::ErrorResponse, helpers::id::generate_id};
use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::{offset, DateTime};
use prisma_client_rust::query_core::schema_builder::constants::filters;

use crate::prisma::{
    key_result::{self, SetParam, WhereParam},
    PrismaClient,
};

use crate::prisma::{objective, user};

use super::model::response::{keyresult_select, KrSelect};

#[derive(Clone)]
pub struct KeyResultService {
    pub db: Arc<PrismaClient>,
    salt: SaltString,
}

impl KeyResultService {
    pub fn init(db: &Arc<PrismaClient>) -> Self {
        Self {
            db: db.clone(),
            salt: SaltString::generate(&mut OsRng),
        }
    }

    pub async fn get_krs(
        &self,
        filters: Vec<WhereParam>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<KrSelect>, ErrorResponse> {
        let krs = self
            .db
            .key_result()
            .find_many(filters)
            .skip(offset)
            .take(limit)
            .select(keyresult_select::select())
            .exec()
            .await?;

        Ok(krs)
    }

    pub async fn get_kr_by_id(&self, kr_id: String) -> Result<KrSelect, ErrorResponse> {
        let kr = self
            .db
            .key_result()
            .find_unique(key_result::pk_kr_id::equals(kr_id))
            .select(keyresult_select::select())
            .exec()
            .await?
            .ok_or_else(|| ErrorResponse::NotFound)?;

        Ok(kr)
    }

    pub async fn create_kr(
        &self,
        name: String,
        user_id: String,
        obj_id: String,
        target: String,
        description: String,
        deadline: i64,
        metric: String,
        params: Vec<SetParam>,
    ) -> Result<KrSelect, ErrorResponse> {
        let deadline_tz = DateTime::from_timestamp(deadline, 0)
            .unwrap()
            .fixed_offset();

        self.db
            .key_result()
            .create(
                generate_id(),
                objective::pk_objective_id::equals(obj_id),
                name,
                description,
                target,
                metric,
                deadline_tz,
                user::pk_user_id::equals(user_id),
                params,
            )
            .select(keyresult_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn update_kr(
        &self,
        kr_id: String,
        changes: Vec<SetParam>,
    ) -> Result<KrSelect, ErrorResponse> {
        self.db
            .key_result()
            .update(key_result::pk_kr_id::equals(kr_id), changes)
            .select(keyresult_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn delete_kr(&self, kr_id: String) -> Result<KrSelect, ErrorResponse> {
        let deleted_kr = self
            .db
            .key_result()
            .delete(key_result::pk_kr_id::equals(kr_id))
            .select(keyresult_select::select())
            .exec()
            .await?;

        Ok(deleted_kr)
    }
}
