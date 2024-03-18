use crate::prisma::file::created_at::equals;
use crate::prisma::{department, organize, period, user};
use crate::prisma::{objective_on_department, objective_on_org, objective_on_user};
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

use super::model::response::{
    obj_id_on_department_select, obj_id_on_org, obj_id_on_user, objective_select, ObjSelect,
};

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
        period_id: String,
        supervisor_id: String,
        params: Vec<SetParam>,
    ) -> Result<ObjSelect, ErrorResponse> {
        let deadline_tz = DateTime::from_timestamp(deadline, 0)
            .unwrap()
            .fixed_offset();

        self.db
            .objective()
            .create(
                generate_id(),
                period::pk_period_id::equals(period_id),
                user::pk_user_id::equals(supervisor_id),
                name,
                target,
                deadline_tz,
                params,
            )
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

    pub async fn add_to_department(
        &self,
        obj_id: String,
        department_id: String,
    ) -> Result<(), ErrorResponse> {
        self.db
            .objective_on_department()
            .create(
                objective::pk_objective_id::equals(obj_id),
                department::pk_department_id::equals(department_id),
                vec![],
            )
            .exec()
            .await?;

        Ok(())
    }

    pub async fn add_to_user(&self, obj_id: String, user_id: String) -> Result<(), ErrorResponse> {
        self.db
            .objective_on_user()
            .create(
                objective::pk_objective_id::equals(obj_id),
                user::pk_user_id::equals(user_id),
                vec![],
            )
            .exec()
            .await?;

        Ok(())
    }

    pub async fn add_to_org(&self, obj_id: String, org_id: String) -> Result<(), ErrorResponse> {
        self.db
            .objective_on_org()
            .create(
                objective::pk_objective_id::equals(obj_id),
                organize::pk_organize_id::equals(org_id),
                vec![],
            )
            .exec()
            .await?;
        Ok(())
    }

    pub async fn get_objs_by_department(
        &self,
        department_id: String,
    ) -> Result<Vec<ObjSelect>, ErrorResponse> {
        let obj_ids: Vec<String> = self
            .db
            .objective_on_department()
            .find_many(vec![objective_on_department::department_id::equals(
                department_id,
            )])
            .select(obj_id_on_department_select::select())
            .exec()
            .await?
            .into_iter()
            .map(|i| i.obj_id)
            .collect();
        let mut objs = vec![];

        for id in obj_ids {
            let obj = Self::get_obj_by_id(self, id).await?;
            objs.push(obj);
        }

        Ok(objs)
    }
    pub async fn get_objs_by_org(&self, org_id: String) -> Result<Vec<ObjSelect>, ErrorResponse> {
        let obj_ids: Vec<String> = self
            .db
            .objective_on_org()
            .find_many(vec![objective_on_org::org_id::equals(org_id)])
            .select(obj_id_on_org::select())
            .exec()
            .await?
            .into_iter()
            .map(|i| i.obj_id)
            .collect();
        let mut objs = vec![];

        for id in obj_ids {
            let obj = Self::get_obj_by_id(self, id).await?;
            objs.push(obj);
        }

        Ok(objs)
    }
    pub async fn get_objs_by_user(&self, user_id: String) -> Result<Vec<ObjSelect>, ErrorResponse> {
        let obj_ids: Vec<String> = self
            .db
            .objective_on_user()
            .find_many(vec![objective_on_user::user_id::equals(user_id)])
            .select(obj_id_on_user::select())
            .exec()
            .await?
            .into_iter()
            .map(|i| i.obj_id)
            .collect();
        let mut objs = vec![];

        for id in obj_ids {
            let obj = Self::get_obj_by_id(self, id).await?;
            objs.push(obj);
        }

        Ok(objs)
    }
}
