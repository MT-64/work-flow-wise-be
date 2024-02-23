pub mod create;
pub mod delete;
pub mod update;

use crate::state::AppState;
use axum::Router;

use self::{
    create::create_organize,
    delete::delete_organize,
    //    get::{get_department, get_departments},
    update::update_organize,
};

pub fn organize_routes() -> Router<AppState> {
    Router::new().nest(
        "/api/v1/organize",
        Router::new()
            // .merge(get_organize())
            // .merge(get_departments())
            .merge(create_organize())
            .merge(update_organize())
            .merge(delete_organize()),
    )
}
