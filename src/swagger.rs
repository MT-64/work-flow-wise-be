use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};

use crate::users::model::request::{
    CreateUserRequest, DeleteUserRequest, LoginRequest, UpdateUserRequest,
};

use crate::objectives::model::request::{CreateObjRequest, UpdateObjRequest};

use crate::objectives::model::response::ObjectiveResponse;
use crate::users::model::response::UserResponse;

use crate::objectives;
use crate::users;

#[derive(OpenApi)]
#[openapi(
  info(
    title = "OKR Gateway",
    version = "0.1.0",
  ),
  tags(
    (name = "User", description = "User API"),
  ),
  components(
    schemas(
      // Requests
      // User
      CreateUserRequest,
      UpdateUserRequest,
      DeleteUserRequest,
      LoginRequest,
      // Objective
      CreateObjRequest,
      UpdateObjRequest,
      // Responses
      UserResponse,
      ObjectiveResponse

    )
  ),
  paths(
    /////// user
    users::controller::get::get_users,
    users::controller::get::get_user,
 //   users::login::login,
    users::controller::profile::profile,
    users::controller::create::create_user,
    users::controller::update::update_user,
    users::controller::delete::delete_user,
    users::controller::login::login,

    /////////// objective
    objectives::controller::get::get_obj,
    objectives::controller::get::get_objs,
    objectives::controller::create::create_obj,
    objectives::controller::delete::delete_obj,
    objectives::controller::update::update_obj,


  ),
  modifiers(
    &SecurityAddon
  ),
  security(
    ("Access Token" = []),
    ("Refresh Token" = [])
  ),
)]
pub struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Access Token",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
            components.add_security_scheme(
                "Refresh Token",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-auth-refresh-token"))),
            );
        }
    }
}
