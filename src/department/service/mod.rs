use crate::{error::ErrorResponse, helpers::id::generate_id, prisma::objective_on_department};
use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::{offset, DateTime};
use prisma_client_rust::query_core::schema_builder::constants::filters;

use crate::prisma::{
    department::{self, SetParam, WhereParam},
    organize, PrismaClient,
};

use super::model::response::{department_id_on_obj, department_select, DepartmentSelect};

#[derive(Clone)]
pub struct DepartmentService {
    pub db: Arc<PrismaClient>,
    salt: SaltString,
}

impl DepartmentService {
    pub fn init(db: &Arc<PrismaClient>) -> Self {
        Self {
            db: db.clone(),
            salt: SaltString::generate(&mut OsRng),
        }
    }

    pub async fn get_departments(
        &self,
        filters: Vec<WhereParam>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<DepartmentSelect>, ErrorResponse> {
        let departments = self
            .db
            .department()
            .find_many(filters)
            .skip(offset)
            .take(limit)
            .select(department_select::select())
            .exec()
            .await?;

        Ok(departments)
    }

    pub async fn get_department_by_id(
        &self,
        department_id: String,
    ) -> Result<DepartmentSelect, ErrorResponse> {
        let deparment = self
            .db
            .department()
            .find_unique(department::pk_department_id::equals(department_id))
            .select(department_select::select())
            .exec()
            .await?
            .ok_or_else(|| ErrorResponse::NotFound)?;

        Ok(deparment)
    }

    pub async fn create_department(
        &self,
        organize_id: String,
        name: String,
        params: Vec<SetParam>,
    ) -> Result<DepartmentSelect, ErrorResponse> {
        self.db
            .department()
            .create(
                generate_id(),
                name,
                organize::pk_organize_id::equals(organize_id),
                params,
            )
            .select(department_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn update_department(
        &self,
        department_id: String,
        changes: Vec<SetParam>,
    ) -> Result<DepartmentSelect, ErrorResponse> {
        self.db
            .department()
            .update(department::pk_department_id::equals(department_id), changes)
            .select(department_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn delete_department(
        &self,
        department_id: String,
    ) -> Result<DepartmentSelect, ErrorResponse> {
        let deleted_department = self
            .db
            .department()
            .delete(department::pk_department_id::equals(department_id))
            .select(department_select::select())
            .exec()
            .await?;

        Ok(deleted_department)
    }
    pub async fn get_departments_by_obj(
        &self,
        obj_id: String,
    ) -> Result<Vec<DepartmentSelect>, ErrorResponse> {
        let department_ids: Vec<String> = self
            .db
            .objective_on_department()
            .find_many(vec![objective_on_department::obj_id::equals(obj_id)])
            .select(department_id_on_obj::select())
            .exec()
            .await?
            .into_iter()
            .map(|i| i.department_id)
            .collect();
        let mut departments = vec![];

        for id in department_ids {
            let department = Self::get_department_by_id(self, id).await?;
            departments.push(department);
        }

        Ok(departments)
    }
}
