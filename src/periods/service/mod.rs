use crate::{
    error::ErrorResponse,
    helpers::id::generate_id,
    prisma::{objective::deadline, organize},
};
use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::{offset, DateTime};
use prisma_client_rust::query_core::schema_builder::constants::filters;

use crate::prisma::{
    period::{self, SetParam, WhereParam},
    PrismaClient,
};

use super::model::response::{period_select, PeriodSelect};

#[derive(Clone)]
pub struct PeriodService {
    pub db: Arc<PrismaClient>,
    salt: SaltString,
}

impl PeriodService {
    pub fn init(db: &Arc<PrismaClient>) -> Self {
        Self {
            db: db.clone(),
            salt: SaltString::generate(&mut OsRng),
        }
    }

    pub async fn get_periods(
        &self,
        filters: Vec<WhereParam>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<PeriodSelect>, ErrorResponse> {
        let periods = self
            .db
            .period()
            .find_many(filters)
            .skip(offset)
            .take(limit)
            .select(period_select::select())
            .exec()
            .await?;

        Ok(periods)
    }

    pub async fn get_period_by_id(&self, period_id: String) -> Result<PeriodSelect, ErrorResponse> {
        let period = self
            .db
            .period()
            .find_unique(period::pk_period_id::equals(period_id))
            .select(period_select::select())
            .exec()
            .await?
            .ok_or_else(|| ErrorResponse::NotFound)?;

        Ok(period)
    }

    pub async fn create_period(
        &self,
        name: String,
        organize_id: String,
        start_date: i64,
        end_date: i64,
        params: Vec<SetParam>,
    ) -> Result<PeriodSelect, ErrorResponse> {
        let start_date = DateTime::from_timestamp(start_date, 0)
            .unwrap()
            .fixed_offset();

        let end_date = DateTime::from_timestamp(end_date, 0)
            .unwrap()
            .fixed_offset();

        self.db
            .period()
            .create(
                generate_id(),
                name,
                start_date,
                end_date,
                organize::pk_organize_id::equals(organize_id),
                params,
            )
            .select(period_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn update_period(
        &self,
        period_id: String,
        changes: Vec<SetParam>,
    ) -> Result<PeriodSelect, ErrorResponse> {
        self.db
            .period()
            .update(period::pk_period_id::equals(period_id), changes)
            .select(period_select::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn delete_period(&self, period_id: String) -> Result<PeriodSelect, ErrorResponse> {
        let deleted_period = self
            .db
            .period()
            .delete(period::pk_period_id::equals(period_id))
            .select(period_select::select())
            .exec()
            .await?;

        Ok(deleted_period)
    }
}
