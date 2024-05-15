use crate::{error::ErrorResponse, helpers::id::generate_id};
use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::{offset, DateTime};
use prisma_client_rust::query_core::schema_builder::constants::filters;

use crate::prisma::{
    file_shared::{self, SetParam as FileSetParam},
    key_result::{self, SetParam, WhereParam},
    PrismaClient,
};

use crate::prisma::{objective, user};

use super::model::response::{file_shared_select, keyresult_select, FileSharedSelect, KrSelect};

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
        target: f64,
        expected: f64,
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
                expected,
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

    pub async fn add_file_to_kr(
        &self,
        kr_id: String,
        file_path: String,
        virtual_path: String,
        params: Vec<FileSetParam>,
    ) -> Result<FileSharedSelect, ErrorResponse> {
        let kr = self
            .db
            .key_result()
            .find_unique(key_result::pk_kr_id::equals(kr_id.clone()))
            .select(keyresult_select::select())
            .exec()
            .await?;

        let fileshared = self
            .db
            .file_shared()
            .create(
                file_path,
                virtual_path,
                key_result::UniqueWhereParam::PkKrIdEquals(kr_id),
                params,
            )
            .select(file_shared_select::select())
            .exec()
            .await?;

        Ok(fileshared)
    }
    pub async fn get_files_by_kr_id(
        &self,
        kr_id: String,
    ) -> Result<Vec<FileSharedSelect>, ErrorResponse> {
        let files = self
            .db
            .file_shared()
            .find_many(vec![file_shared::key_result_id::equals(kr_id)])
            .select(file_shared_select::select())
            .exec()
            .await?;

        Ok(files)
    }
}
