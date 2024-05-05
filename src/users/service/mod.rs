use crate::{
    error::ErrorResponse,
    helpers::id::generate_id,
    prisma::{self, objective_on_department::department_id, objective_on_user},
    users::model::response::{
        user_select, user_select_with_password, UserSelect, UserSelectWithPassword,
    },
};
use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use prisma_client_rust::PrismaValue;

use crate::prisma::{
    user::{self, SetParam, WhereParam},
    PrismaClient,
};

use super::model::response::user_id_on_obj;

#[derive(Clone)]
pub struct UserService {
    pub db: Arc<PrismaClient>,
    salt: SaltString,
}

impl UserService {
    pub fn init(db: &Arc<PrismaClient>) -> Self {
        Self {
            db: db.clone(),
            salt: SaltString::generate(&mut OsRng),
        }
    }

    pub async fn get_users(
        &self,
        filters: Vec<WhereParam>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<UserSelect>, ErrorResponse> {
        let users = self
            .db
            .user()
            .find_many(filters)
            .skip(offset)
            .take(limit)
            .select(user_select::select())
            .exec()
            .await?;
        Ok(users)
    }

    pub async fn get_user_by_login_info(
        &self,
        username: String,
        password: String,
    ) -> Result<UserSelect, ErrorResponse> {
        let user = self
            .db
            .user()
            .find_first(vec![
                user::username::equals(username.clone()),
                //                user::password::equals(password),
            ])
            .select(user_select_with_password::select())
            .exec()
            .await?
            .ok_or_else(|| ErrorResponse::NotFound)?;

        match argon2::Argon2::verify_password(
            &Argon2::default(),
            password.as_bytes(),
            &user
                .password
                .as_str()
                .try_into()
                .expect("password hash error"),
        ) {
            Ok(()) => {
                return self
                    .db
                    .user()
                    .find_first(vec![user::username::equals(username)])
                    .select(user_select::select())
                    .exec()
                    .await?
                    .ok_or_else(|| ErrorResponse::NotFound);
            }
            Err(_) => return Err(ErrorResponse::NotFound),
        }
    }

    pub async fn get_user_by_id(&self, user_id: String) -> Result<UserSelect, ErrorResponse> {
        let user = self
            .db
            .user()
            .find_unique(user::pk_user_id::equals(user_id))
            .select(user_select::select())
            .exec()
            .await?
            .ok_or_else(|| ErrorResponse::NotFound)?;

        Ok(user)
    }

    pub async fn get_user_by_id_with_password(
        &self,
        user_id: String,
    ) -> Result<UserSelectWithPassword, ErrorResponse> {
        let user = self
            .db
            .user()
            .find_unique(user::pk_user_id::equals(user_id))
            .select(user_select_with_password::select())
            .exec()
            .await?
            .ok_or_else(|| ErrorResponse::NotFound)?;

        Ok(user)
    }

    pub async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
        params: Vec<SetParam>,
    ) -> Result<UserSelect, ErrorResponse> {
        let password = Argon2::default()
            .hash_password(password.as_bytes(), &self.salt)?
            .to_string();
        self.db
            .user()
            .create(generate_id(), username, email, password, params)
            .select(user_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn update_user(
        &self,
        user_id: String,
        changes: Vec<SetParam>,
    ) -> Result<UserSelect, ErrorResponse> {
        self.db
            .user()
            .update(user::pk_user_id::equals(user_id), changes)
            .select(user_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn delete_user(&self, user_id: String) -> Result<UserSelect, ErrorResponse> {
        let deleted_user = self
            .db
            .user()
            .delete(user::pk_user_id::equals(user_id))
            .select(user_select::select())
            .exec()
            .await?;

        Ok(deleted_user)
    }
    pub async fn add_user_to_department(
        &self,
        user_id: String,
        department_id: String,
    ) -> Result<UserSelect, ErrorResponse> {
        let changes: Vec<SetParam> = vec![user::department_id::set(Some(department_id))];

        self.update_user(user_id, changes).await
    }
    pub async fn remove_user_department(
        &self,
        user_id: String,
    ) -> Result<UserSelect, ErrorResponse> {
        let changes: Vec<SetParam> = vec![user::department_id::set(Some(
            PrismaValue::Null.to_string(),
        ))];

        self.update_user(user_id, changes).await
    }

    pub async fn add_user_to_organize(
        &self,
        user_id: String,
        org_id: String,
    ) -> Result<UserSelect, ErrorResponse> {
        let changes: Vec<SetParam> = vec![user::organize_id::set(Some(org_id))];

        self.update_user(user_id, changes).await
    }

    pub async fn get_users_by_obj(&self, obj_id: String) -> Result<Vec<UserSelect>, ErrorResponse> {
        let user_ids: Vec<String> = self
            .db
            .objective_on_user()
            .find_many(vec![objective_on_user::obj_id::equals(obj_id)])
            .select(user_id_on_obj::select())
            .exec()
            .await?
            .into_iter()
            .map(|i| i.user_id)
            .collect();
        let mut users = vec![];

        for id in user_ids {
            let user = Self::get_user_by_id(self, id).await?;
            users.push(user);
        }

        Ok(users)
    }
}
