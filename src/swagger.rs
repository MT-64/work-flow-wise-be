use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};

use crate::users::model::request::{
    CreateUserRequest, DeleteUserRequest, LoginRequest, UpdateUserRequest, UserQueryRequest, AddMultipleUserToDepartment, AddMultipleUserToOrg,
};

use crate::department::model::request::{CreateDepartmentRequest, UpdateDepartmentRequest};
use crate::key_result::model::request::{CreateKrRequest, UpdateKrRequest, GradingKr};
use crate::objectives::model::request::{CreateObjRequest, UpdateObjRequest};
use crate::organize::model::request::{CreateOrganizeRequest, UpdateOrganizeRequest};
use crate::periods::model::request::{CreatePeriodRequest, UpdatePeriodRequest};
use crate::notification::model::request::NotificationQueryRequest;

use crate::department::model::response::DepartmentResponse;
use crate::key_result::model::response::KeyResultResponse;
use crate::objectives::model::response::{ObjectiveResponse, ProgressResponse};
use crate::organize::model::response::OrganizeResponse;
use crate::periods::model::response::PeriodResponse;
use crate::users::model::response::UserResponse;
use crate::file::model::{select::File, query::FileQuery};
use crate::notification::model::response::NotificationResponse;

use crate::department;
use crate::key_result;
use crate::objectives;
use crate::organize;
use crate::periods;
use crate::users;
use crate::file;
use crate::folder;
use crate::notification;

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
      UserQueryRequest, 
      AddMultipleUserToDepartment, 
      AddMultipleUserToOrg,
      // Objective
      CreateObjRequest,
      UpdateObjRequest,
      // Department
      CreateDepartmentRequest,
      UpdateDepartmentRequest,
      //Keyresult
      CreateKrRequest,
      UpdateKrRequest,
      GradingKr,
      // Period
      CreatePeriodRequest,
      UpdatePeriodRequest,
      // Organize
      CreateOrganizeRequest,
      UpdateOrganizeRequest,
      //File 

      // Notification
      NotificationQueryRequest,
      // Responses
      UserResponse,
      ObjectiveResponse,
      DepartmentResponse,
      KeyResultResponse,
      PeriodResponse,
      ProgressResponse,
      OrganizeResponse,
      NotificationResponse

    )
  ),
  paths(
    /////// user
    users::controller::get::get_users,
    users::controller::get::get_user,
    users::controller::get::get_users_by_obj,
 //   users::login::login,
    users::controller::profile::profile,
    users::controller::create::create_user,
    users::controller::update::update_user,
    users::controller::delete::delete_user,
    users::controller::login::login,
    users::controller::add_to_department::add_to_department,
    users::controller::add_to_department::add_multiple_to_department,
    users::controller::add_to_org::add_to_organize,
    users::controller::add_to_org::add_multiple_to_org,
    users::controller::get::get_user_by_jwt,

    /////////// objective
    objectives::controller::get::get_obj,
    objectives::controller::get::get_objs,
    objectives::controller::get::get_objs_by_department,
    objectives::controller::get::get_objs_by_org,
    objectives::controller::get::get_objs_by_user,
    objectives::controller::create::create_obj,
    objectives::controller::delete::delete_obj,
    objectives::controller::update::update_obj,
    objectives::controller::add_to_department::add_to_department,
    objectives::controller::add_to_user::add_to_user,
    objectives::controller::add_to_org::add_to_organize,
    objectives::controller::get::get_objs_by_parent,
    objectives::controller::get::get_obj_progress,
    
    ////////// department
    department::controller::get::get_department,
    department::controller::get::get_departments,
    department::controller::create::create_department,
    department::controller::delete::delete_department,
    department::controller::update::update_department,

    ////////// keyresult
    key_result::controller::get::get_krs,
    key_result::controller::get::get_kr,
    key_result::controller::create::create_kr,
    key_result::controller::delete::delete_kr,
    key_result::controller::update::update_kr,
    key_result::controller::update::grading_kr,

    ////////// period
    periods::controller::get::get_periods,
    periods::controller::get::get_period,
    periods::controller::create::create_period,
    periods::controller::delete::delete_period,
    periods::controller::update::update_period,
    
    ////////// organize
    organize::controller::get::get_organize,
    organize::controller::get::get_organizes,
    organize::controller::create::create_organize,
    organize::controller::delete::delete_organize,
    organize::controller::update::update_organize,

    //////// file 
    file::controller::get::my::get_my_files,
    /////// notification
    notification::controller::get::get_noties,
    notification::controller::update::update_noti,




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
