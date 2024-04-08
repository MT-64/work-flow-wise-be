pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use crate::state::AppState;
use axum::Router;

use self::{
    create::create_organize,
    delete::delete_organize,
    get::{get_organize, get_organizes, get_orgs_by_obj},
    update::update_organize,
};

pub fn organize_routes() -> Router<AppState> {
    Router::new().nest(
        "/api/v1/organize",
        Router::new()
            .merge(get_organize())
            .merge(get_organizes())
            .merge(create_organize())
            .merge(update_organize())
            .merge(get_orgs_by_obj())
            .merge(delete_organize()),
    )
}
