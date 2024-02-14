use axum::{extract::State, routing::options, Router};

use crate::{
    department::controller::department_routes, objectives::controller::obj_routes,
    response::WebResponse, state::AppState, users::controller::user_routes, WebResult,
};

fn preflight() -> Router<AppState> {
    async fn preflight_handler(_: State<AppState>) -> WebResult {
        Ok(WebResponse::ok("Preflight request passed", ()))
    }
    Router::new().route("/", options(preflight_handler))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(preflight())
        .merge(user_routes())
        .merge(obj_routes())
        .merge(department_routes())
    // .merge(auth_routes())
    // .merge(folder_routes())
    // .merge(file_routes())
    // .merge(tag_route())
}
