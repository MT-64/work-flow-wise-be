use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

//use crate::env::{hostname, port};

use crate::prisma::user;

user::select!(user_select {
    pk_user_id
    pagination_id
    username
    first_name
    last_name
    role
    gender
    introduction_brief
    image
    total_credit
    email
    created_at
    updated_at
});

user::select!(user_select_with_password {
    pk_user_id
    password
});

pub type UserSelect = user_select::Data;
pub type UserSelectWithPassword = user_select_with_password::Data;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub enum Role {
    Subscriber = 0,
    Instructor = 1,
    Moderator = 2,
    Admin = 3,
}

impl From<i32> for Role {
    fn from(role: i32) -> Self {
        match role {
            1 => Self::Instructor,
            2 => Self::Moderator,
            3 => Self::Admin,
            _ => Self::Subscriber,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub email: String,
    pub role: String,
    pub gender: String,
    pub introduction_brief: Option<String>,
    pub image: Option<String>,
    pub total_credit: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<UserSelect> for UserResponse {
    fn from(
        UserSelect {
            pk_user_id,
            pagination_id,
            username,
            first_name,
            last_name,
            role,
            gender,
            introduction_brief,
            image,
            total_credit,
            email,
            created_at,
            updated_at,
        }: UserSelect,
    ) -> Self {
        Self {
            id: pk_user_id,
            first_name,
            last_name,
            username,
            email,
            role: role.to_string(),
            gender: gender.to_string(),
            introduction_brief,
            image,
            total_credit,
            created_at: created_at.with_timezone(&Utc).timestamp(),
            updated_at: updated_at.with_timezone(&Utc).timestamp(),
        }
    }
}
