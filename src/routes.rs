use axum::{extract::State, routing::options, Router};

use crate::{
    department::controller::department_routes, key_result::controller::kr_routes,
    objectives::controller::obj_routes, organize::controller::organize_routes,
    periods::controller::period_routes, response::WebResponse, state::AppState,
    users::controller::user_routes, WebResult,
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
        .merge(kr_routes())
        .merge(period_routes())
        .merge(organize_routes())
    // .merge(auth_routes())
    // .merge(folder_routes())
    // .merge(file_routes())
    // .merge(tag_route())
}
