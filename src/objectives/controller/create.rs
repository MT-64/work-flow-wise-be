use axum::{extract::State, routing::post, Router};

use crate::{
    file::model::select::child_files_select::child_files,
    objectives::model::{request::CreateObjRequest, response::ObjectiveResponse},
    prisma::{
        self,
        objective::{self, obj_for},
    },
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
        State(AppState {
            obj_service,
            user_service,
            department_service,
            ..
        }): State<AppState>,
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
            metric,
            obj_for,
            expected,
            child_ids,
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

        let new_obj_for = match obj_for.clone().trim() {
            "User" => prisma::ObjectiveFor::User,
            "Department" => prisma::ObjectiveFor::Department,
            "Organize" => prisma::ObjectiveFor::Organize,
            _ => unreachable!(),
        };

        let new_metric = match metric.trim() {
            "Quantity" => prisma::ObjectiveMetric::Quantity,
            "Percent" => prisma::ObjectiveMetric::Percent,
            "Time" => prisma::ObjectiveMetric::Time,
            "Money" => prisma::ObjectiveMetric::Money,
            _ => unreachable!(),
        };

        match parent_objective_id {
            Some(ref parent_id) => {
                let parent_obj = obj_service.get_obj_by_id(parent_id.clone()).await?;
                let _ = obj_service
                    .update_obj(
                        parent_id.clone(),
                        vec![objective::target::set(parent_obj.target + target.clone())],
                    )
                    .await?;
            }
            None => {}
        }
        params.push(objective::parent_objective_id::set(parent_objective_id));

        params.push(objective::description::set(description));
        params.push(objective::progress::set(progress));
        params.push(objective::obj_type::set(new_obj_type));
        let new_obj: ObjectiveResponse = obj_service
            .create_obj(
                expected,
                name,
                target,
                deadline,
                period_id,
                supervisor_id,
                new_obj_for.clone(),
                new_metric,
                params,
            )
            .await?
            .into();
        match new_obj_for {
            crate::prisma::ObjectiveFor::User => {
                for id in child_ids {
                    let _ = obj_service.add_to_user(new_obj.obj_id.clone(), id).await?;
                }
            }
            crate::prisma::ObjectiveFor::Department => {
                for user_id in child_ids.iter() {
                    let _ = obj_service
                        .add_to_user(new_obj.obj_id.clone(), user_id.to_string())
                        .await?;
                }
                for user_id in child_ids.iter() {
                    let user = user_service.get_user_by_id(user_id.to_string()).await?;
                    match user.department_id {
                        Some(department_id) => {
                            let _ = obj_service
                                .add_to_department(new_obj.obj_id.clone(), department_id)
                                .await?;
                        }
                        None => continue,
                    }
                }
            }
            crate::prisma::ObjectiveFor::Organize => {
                for department_id in child_ids.iter() {
                    let _ = obj_service
                        .add_to_department(new_obj.obj_id.clone(), department_id.to_string())
                        .await?;
                }
                if !child_ids.is_empty() && child_ids.first().is_some() {
                    let department = department_service
                        .get_department_by_id(child_ids.first().unwrap().to_string())
                        .await?;
                    let _ = obj_service
                        .add_to_org(new_obj.obj_id.clone(), department.organize_id)
                        .await?;
                }
            }
        }

        Ok(WebResponse::created(
            "Created objective sucessfully",
            ObjectiveResponse::from(new_obj),
        ))
    }
    Router::new().route("/create", post(create_obj_handler))
}
