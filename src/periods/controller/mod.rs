use axum::Router;

use crate::state::AppState;

use self::{
    create::create_period,
    delete::delete_period,
    get::{get_period, get_periods},
    update::update_period,
};

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub fn period_routes() -> Router<AppState> {
    Router::new().nest(
        "/api/v1/period",
        Router::new()
            .merge(get_periods())
            .merge(get_period())
            .merge(create_period())
            .merge(delete_period())
            .merge(update_period()),
    )
}
