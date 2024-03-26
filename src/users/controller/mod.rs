use axum::Router;

use crate::state::AppState;

use self::{
    add_to_department::add_to_department,
    add_to_org::add_to_organize,
    create::create_user,
    delete::delete_user,
    get::{get_user, get_users, get_users_by_obj},
    login::login,
    profile::profile,
    update::update_user,
};

pub mod add_to_department;
pub mod add_to_org;
pub mod create;
pub mod delete;
pub mod get;
pub mod login;
pub mod profile;
pub mod update;

pub fn user_routes() -> Router<AppState> {
    Router::new().nest(
        "/api/v1/user",
        Router::new()
            .merge(get_users())
            .merge(get_user())
            .merge(profile())
            .merge(create_user())
            .merge(update_user())
            .merge(delete_user())
            .merge(login())
            .merge(get_users_by_obj())
            .merge(add_to_department())
            .merge(add_to_organize()),
    )
}
