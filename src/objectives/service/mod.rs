use crate::{error::ErrorResponse, helpers::id::generate_id, prisma::objective::deadline};
use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::{offset, DateTime};
use prisma_client_rust::query_core::schema_builder::constants::filters;

use crate::prisma::{
    objective::{self, SetParam, WhereParam},
    PrismaClient,
};

use super::model::response::{objective_select, ObjSelect};

#[derive(Clone)]
pub struct ObjectiveService {
    pub db: Arc<PrismaClient>,
    salt: SaltString,
}

impl ObjectiveService {
    pub fn init(db: &Arc<PrismaClient>) -> Self {
        Self {
            db: db.clone(),
            salt: SaltString::generate(&mut OsRng),
        }
    }

    pub async fn get_objs(
        &self,
        filters: Vec<WhereParam>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<ObjSelect>, ErrorResponse> {
        let objs = self
            .db
            .objective()
            .find_many(filters)
            .skip(offset)
            .take(limit)
            .select(objective_select::select())
            .exec()
            .await?;

        Ok(objs)
    }

    pub async fn get_obj_by_id(&self, obj_id: String) -> Result<ObjSelect, ErrorResponse> {
        let obj = self
            .db
            .objective()
            .find_unique(objective::pk_objective_id::equals(obj_id))
            .select(objective_select::select())
            .exec()
            .await?
            .ok_or_else(|| ErrorResponse::NotFound)?;

        Ok(obj)
    }

    pub async fn create_obj(
        &self,
        name: String,
        target: String,
        deadline: i64,
        params: Vec<SetParam>,
    ) -> Result<ObjSelect, ErrorResponse> {
        let deadline_tz = DateTime::from_timestamp(deadline, 0)
            .unwrap()
            .fixed_offset();

        self.db
            .objective()
            .create(generate_id(), name, target, deadline_tz, params)
            .select(objective_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn update_obj(
        &self,
        obj_id: String,
        changes: Vec<SetParam>,
    ) -> Result<ObjSelect, ErrorResponse> {
        self.db
            .objective()
            .update(objective::pk_objective_id::equals(obj_id), changes)
            .select(objective_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn delete_obj(&self, obj_id: String) -> Result<ObjSelect, ErrorResponse> {
        let deleted_obj = self
            .db
            .objective()
            .delete(objective::pk_objective_id::equals(obj_id))
            .select(objective_select::select())
            .exec()
            .await?;

        Ok(deleted_obj)
    }
}
