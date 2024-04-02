use crate::{error::ErrorResponse, helpers::id::generate_id};
use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::{offset, DateTime};
use prisma_client_rust::query_core::schema_builder::constants::filters;

use crate::prisma::{
    notification::{self, SetParam, WhereParam},
    PrismaClient,
};

use crate::prisma::{objective, user};

use super::model::response::{notification_select, NotiSelect};

#[derive(Clone)]
pub struct NotificationService {
    pub db: Arc<PrismaClient>,
    salt: SaltString,
}

impl NotificationService {
    pub fn init(db: &Arc<PrismaClient>) -> Self {
        Self {
            db: db.clone(),
            salt: SaltString::generate(&mut OsRng),
        }
    }

    pub async fn get_noties(
        &self,
        filters: Vec<WhereParam>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<NotiSelect>, ErrorResponse> {
        let noties = self
            .db
            .notification()
            .find_many(filters)
            .skip(offset)
            .take(limit)
            .select(notification_select::select())
            .exec()
            .await?;

        Ok(noties)
    }

    pub async fn get_note_by_id(&self, noti_id: String) -> Result<NotiSelect, ErrorResponse> {
        let noti = self
            .db
            .notification()
            .find_unique(notification::pk_notification_id::equals(noti_id))
            .select(notification_select::select())
            .exec()
            .await?
            .ok_or_else(|| ErrorResponse::NotFound)?;

        Ok(noti)
    }

    pub async fn create_noti(
        &self,
        user_id: String,
        message: String,
        params: Vec<SetParam>,
    ) -> Result<NotiSelect, ErrorResponse> {
        self.db
            .notification()
            .create(generate_id(), user_id, message, params)
            .select(notification_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn update_noti(
        &self,
        noti_id: String,
        changes: Vec<SetParam>,
    ) -> Result<NotiSelect, ErrorResponse> {
        self.db
            .notification()
            .update(notification::pk_notification_id::equals(noti_id), changes)
            .select(notification_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn delete_noti(&self, noti_id: String) -> Result<NotiSelect, ErrorResponse> {
        let deleted_noti = self
            .db
            .notification()
            .delete(notification::pk_notification_id::equals(noti_id))
            .select(notification_select::select())
            .exec()
            .await?;

        Ok(deleted_noti)
    }
}
