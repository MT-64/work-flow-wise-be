pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use crate::state::AppState;
use axum::Router;

use self::{
    create::create_department,
    delete::delete_department,
    get::{get_department, get_departments, get_departments_by_obj},
    update::update_department,
};

pub fn department_routes() -> Router<AppState> {
    Router::new().nest(
        "/api/v1/department",
        Router::new()
            .merge(get_department())
            .merge(get_departments())
            .merge(create_department())
            .merge(update_department())
            .merge(get_departments_by_obj())
            .merge(delete_department()),
    )
}
