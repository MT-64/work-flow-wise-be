use axum::Router;

use crate::state::AppState;

use self::{
    add_to_department::add_to_department,
    add_to_org::add_to_organize,
    add_to_user::add_to_user,
    create::create_obj,
    delete::delete_obj,
    get::{get_obj, get_objs},
    update::update_obj,
};

pub mod add_to_department;
pub mod add_to_org;
pub mod add_to_user;
pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub fn obj_routes() -> Router<AppState> {
    Router::new().nest(
        "/api/v1/objective",
        Router::new()
            .merge(get_objs())
            .merge(get_obj())
            .merge(create_obj())
            .merge(update_obj())
            .merge(delete_obj())
            .merge(add_to_department())
            .merge(add_to_user())
            .merge(add_to_organize()),
    )
}
