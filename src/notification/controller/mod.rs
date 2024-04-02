use axum::Router;

use crate::state::AppState;

use self::{get::get_noties, update::update_noti};

pub mod get;
pub mod update;

pub fn noti_routes() -> Router<AppState> {
    Router::new().nest(
        "/api/v1/notification",
        Router::new().merge(get_noties()).merge(update_noti()),
    )
}
