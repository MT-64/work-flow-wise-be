use axum::Router;

use crate::state::AppState;

use self::{
    add_file::add_file,
    create::create_kr,
    delete::delete_kr,
    get::{get_kr, get_krs},
    update::{grading_kr, update_kr},
};

pub mod add_file;
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
            .merge(grading_kr())
            .merge(update_kr())
            .merge(add_file())
            .merge(delete_kr()),
    )
}
