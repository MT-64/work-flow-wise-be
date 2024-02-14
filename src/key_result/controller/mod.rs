use axum::Router;

use crate::state::AppState;

use self::{
    create::create_kr,
    delete::delete_kr,
    get::{get_kr, get_krs},
    update::update_kr,
};

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub fn kr_routes() -> Router<AppState> {
    Router::new().nest(
        "/api/v1/kr",
        Router::new()
            .merge(get_krs())
            .merge(get_kr())
            .merge(create_kr())
            .merge(update_kr())
            .merge(delete_kr()),
    )
}
