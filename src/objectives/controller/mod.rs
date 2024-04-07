use axum::Router;

use crate::state::AppState;

use self::{
    add_to_department::add_to_department,
    add_to_org::add_to_organize,
    add_to_user::add_to_user,
    check_state::check_state_obj,
    create::create_obj,
    delete::delete_obj,
    get::{
        get_obj, get_obj_progress, get_objs, get_objs_by_department, get_objs_by_org,
        get_objs_by_parent, get_objs_by_user,
    },
    update::update_obj,
};

pub mod add_to_department;
pub mod add_to_org;
pub mod add_to_user;
pub mod check_state;
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
            .merge(check_state_obj())
            .merge(add_to_user())
            .merge(get_objs_by_department())
            .merge(get_objs_by_org())
            .merge(get_objs_by_parent())
            .merge(get_objs_by_user())
            .merge(get_obj_progress())
            .merge(add_to_organize()),
    )
}
