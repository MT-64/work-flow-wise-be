use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};

use crate::users::model::request::{
    CreateUserRequest, DeleteUserRequest, LoginRequest, UpdateUserRequest, UserQueryRequest, AddMultipleUserToDepartment, AddMultipleUserToOrg, UpdateRoleRequest
};

use crate::comment::{controller::create::AddCommentToCommentRequest ,model::{CommentResponse, CommentTreeNodeResponse}};
use crate::department::model::request::{CreateDepartmentRequest, UpdateDepartmentRequest};
use crate::key_result::model::request::{CreateKrRequest, UpdateKrRequest, GradingKr, AddFileRequest };
use crate::objectives::model::request::{CreateObjRequest, UpdateObjRequest};
use crate::organize::model::request::{CreateOrganizeRequest, UpdateOrganizeRequest};
use crate::periods::model::request::{CreatePeriodRequest, UpdatePeriodRequest};
use crate::notification::model::request::NotificationQueryRequest;

use crate::department::model::response::DepartmentResponse;
use crate::key_result::model::response::KeyResultResponse;
use crate::key_result::model::request::UpdateKrProgressRequest;
use crate::key_result::model::response::FileSharedResponse;
use crate::objectives::model::response::{ObjectiveResponse, ProgressResponse};
use crate::organize::model::response::OrganizeResponse;
use crate::periods::model::response::PeriodResponse;
use crate::users::model::response::UserResponse;
use crate::file::model::{select::File, query::FileQuery};
use crate::notification::model::response::NotificationResponse;
use crate::file::controller::create::FileUploadedResponse;
use crate::objectives::model::request::CheckStateObjRequest;
use crate::department;
use crate::key_result;
use crate::objectives;
use crate::organize;
use crate::periods;
use crate::users;
use crate::file;
use crate::folder;
use crate::notification;
use crate::comment;

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
      FileSharedResponse,
      CheckStateObjRequest,
      FileUploadedResponse,
      CreateUserRequest,
      UpdateKrProgressRequest,
      UpdateUserRequest,
      UpdateRoleRequest,
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
      AddFileRequest,
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
      NotificationResponse,
      //// Comment
      AddCommentToCommentRequest,
      CommentResponse,
      CommentTreeNodeResponse

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
    users::controller::create::admin_create_user,
    users::controller::update::update_user,
    users::controller::update::update_user_role,
    users::controller::delete::delete_user,
    users::controller::login::login,
    users::controller::add_to_department::add_to_department,
    users::controller::add_to_department::remove_user_department,
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
    objectives::controller::add_to_department::remove_from_department,
    objectives::controller::add_to_user::add_to_user,
    objectives::controller::add_to_user::remove_from_user,
    objectives::controller::add_to_org::add_to_organize,
    objectives::controller::add_to_org::remove_from_org,
    objectives::controller::get::get_objs_by_parent,
    objectives::controller::get::get_obj_progress,
    objectives::controller::check_state::check_state_obj,
    
    ////////// department
    department::controller::get::get_department,
    department::controller::get::get_departments,
    department::controller::get::get_departments_by_obj,
    department::controller::create::create_department,
    department::controller::delete::delete_department,
    department::controller::update::update_department,

    ////////// keyresult
    key_result::controller::get::get_krs,
    key_result::controller::get::get_kr,
    key_result::controller::get::get_kr_file,
    key_result::controller::create::create_kr,
    key_result::controller::delete::delete_kr,
    key_result::controller::update::update_kr,
    key_result::controller::update::update_kr_progress,
    key_result::controller::update::grading_kr,
    key_result::controller::add_file::add_file,

    ////////// period
    periods::controller::get::get_periods,
    periods::controller::get::get_period,
    periods::controller::create::create_period,
    periods::controller::delete::delete_period,
    periods::controller::update::update_period,
    
    ////////// organize
    organize::controller::get::get_organize,
    organize::controller::get::get_organizes,
    organize::controller::get::get_orgs_by_obj,
    organize::controller::create::create_organize,
    organize::controller::delete::delete_organize,
    organize::controller::update::update_organize,

    //////// file 
    file::controller::get::my::get_my_files,
    file::controller::create::upload_file,
    /////// notification
    notification::controller::get::get_noties,
    notification::controller::update::update_noti,

    ///// comment 
    comment::controller::create::add_to_comment,
    comment::controller::get::get_comments_by_post,
    comment::controller::get::get_comment_by_id




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
