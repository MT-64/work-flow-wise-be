use axum::Router;

use crate::state::AppState;

use self::{
    create::create_obj,
    delete::delete_obj,
    get::{get_obj, get_objs},
    update::update_obj,
};

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
            .merge(delete_obj()),
    )
}
