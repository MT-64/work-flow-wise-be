use axum::{extract::State, routing::post, Router};

use crate::{
    objectives::model::{request::CreateObjRequest, response::ObjectiveResponse},
    prisma::{self, objective},
    response::WebResponse,
    state::AppState,
    WebResult,
};

#[utoipa::path(
  post,
  tag = "Objective",
  path = "/api/v1/objective/create",
  request_body(
    content = CreateObjRequest,
    description = "Create Objective Request",
  ),
  responses(
    (
      status = 201,
      description = "Objective created",
      body = ObjectiveResponse,
      example = json! (
        {
          "code": 201,
          "message": "Created new objective successfully",
          "data": {
            "createdAt": 1696932804946_i64,
            "updatedAt": 1696932804946_i64
          },
          "error": ""
        }
      )
    ),
  )
)]
pub fn create_obj() -> Router<AppState> {
    async fn create_obj_handler(
        State(AppState { obj_service, .. }): State<AppState>,
        CreateObjRequest {
            obj_type,
            period_id,
            supervisor_id,
            name,
            description,
            target,
            progress,
            deadline,
            parent_objective_id,
        }: CreateObjRequest,
    ) -> WebResult {
        let mut params = vec![];

        let new_obj_type = match obj_type.trim() {
            "Percent" => prisma::ObjectiveType::Percent,
            "Kpi" => prisma::ObjectiveType::Kpi,
            "As high as possible" => prisma::ObjectiveType::AsHighAsPossible,
            "As low as possible" => prisma::ObjectiveType::AsLowAsPossible,
            _ => prisma::ObjectiveType::Other,
        };

        params.push(objective::parent_objective_id::set(parent_objective_id));

        params.push(objective::description::set(description));
        params.push(objective::progress::set(progress));
        params.push(objective::obj_type::set(new_obj_type));

        let new_obj: ObjectiveResponse = obj_service
            .create_obj(name, target, deadline, period_id, supervisor_id, params)
            .await?
            .into();

        Ok(WebResponse::created(
            "Created objective sucessfully",
            ObjectiveResponse::from(new_obj),
        ))
    }
    Router::new().route("/create", post(create_obj_handler))
}
