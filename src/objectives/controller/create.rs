use axum::{extract::State, routing::post, Router};
use utoipa::openapi::AllOf;

use crate::{
    file::model::select::child_files_select::child_files,
    objectives::model::{request::CreateObjRequest, response::ObjectiveResponse},
    prisma::{
        self,
        objective::{self, obj_for},
        user,
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
            notification_service,
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
        params.push(objective::parent_objective_id::set(
            parent_objective_id.clone(),
        ));

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
                    let _ = obj_service
                        .add_to_user(new_obj.obj_id.clone(), id.clone())
                        .await?;

                    let message = format!(r#"Mục tiêu mới {} được gán cho bạn"#, new_obj.name);
                    notification_service
                        .create_noti(id.to_string(), message.clone(), vec![])
                        .await?;

                    match parent_objective_id {
                        //// update department objective progress
                        Some(ref parent_id) => {
                            let department_obj =
                                obj_service.get_obj_by_id(parent_id.clone()).await?;

                            let all_user_obj = obj_service
                                .get_objs(
                                    vec![objective::parent_objective_id::equals(Some(
                                        parent_id.clone(),
                                    ))],
                                    0,
                                    100,
                                )
                                .await?;
                            let mut progress_department_obj = 0.0;
                            let mut num_obj = 0.0;

                            for obj_user in &all_user_obj {
                                progress_department_obj += obj_user.progress.unwrap_or(0.0);
                                num_obj += 1.0;
                            }
                            if num_obj == 0.0 {
                                num_obj = 1.0;
                            }

                            let _ = obj_service
                                .update_obj(
                                    parent_id.clone(),
                                    vec![objective::progress::set(Some(
                                        (progress_department_obj / num_obj) as f64,
                                    ))],
                                )
                                .await?;
                            //// update organize objective
                            match department_obj.parent_objective_id {
                                Some(parent_department_obj_id) => {
                                    let all_department_obj = obj_service
                                        .get_objs(
                                            vec![objective::parent_objective_id::equals(Some(
                                                parent_department_obj_id.clone(),
                                            ))],
                                            0,
                                            100,
                                        )
                                        .await?;
                                    let mut progress_org_obj = 0.0;
                                    let mut num_department_obj = 0.0;

                                    for obj_department in &all_department_obj {
                                        progress_org_obj += obj_department.progress.unwrap_or(0.0);
                                        num_department_obj += 1.0;
                                    }

                                    if num_department_obj == 0.0 {
                                        num_department_obj = 1.0;
                                    }
                                    let _ = obj_service
                                        .update_obj(
                                            parent_department_obj_id.clone(),
                                            vec![objective::progress::set(Some(
                                                (progress_org_obj / num_department_obj) as f64,
                                            ))],
                                        )
                                        .await?;
                                }
                                None => {}
                            }
                        }
                        None => {}
                    }
                }
            }
            crate::prisma::ObjectiveFor::Department => {
                for user_id in child_ids.iter() {
                    let _ = obj_service
                        .add_to_user(new_obj.obj_id.clone(), user_id.to_string())
                        .await?;

                    let message = format!(r#"Mục tiêu mới {} được gán cho bạn"#, new_obj.name);
                    notification_service
                        .create_noti(user_id.to_string(), message.clone(), vec![])
                        .await?;
                }
                for user_id in child_ids.iter() {
                    let user = user_service.get_user_by_id(user_id.to_string()).await?;
                    match user.department_id {
                        Some(department_id) => {
                            let _ = obj_service
                                .add_to_department(new_obj.obj_id.clone(), department_id)
                                .await?;
                            break;
                        }
                        None => continue,
                    }
                }
                // update organize objective
                match parent_objective_id {
                    //// update department objective progress
                    Some(ref parent_id) => {
                        let org_obj = obj_service.get_obj_by_id(parent_id.clone()).await?;

                        let all_department_obj = obj_service
                            .get_objs(
                                vec![objective::parent_objective_id::equals(Some(
                                    parent_id.clone(),
                                ))],
                                0,
                                100,
                            )
                            .await?;
                        let mut progress_org_obj = 0.0;
                        let mut num_obj = 0.0;

                        for obj_department in &all_department_obj {
                            progress_org_obj += obj_department.progress.unwrap_or(0.0);
                            num_obj += 1.0;
                        }
                        if num_obj == 0.0 {
                            num_obj = 1.0;
                        }

                        let _ = obj_service
                            .update_obj(
                                parent_id.clone(),
                                vec![objective::progress::set(Some(
                                    (progress_org_obj / num_obj) as f64,
                                ))],
                            )
                            .await?;
                    }
                    None => {}
                }
            }
            crate::prisma::ObjectiveFor::Organize => {
                for department_id in child_ids.iter() {
                    let _ = obj_service
                        .add_to_department(new_obj.obj_id.clone(), department_id.to_string())
                        .await?;
                    let users = user_service
                        .get_users(
                            vec![user::department_id::equals(Some(department_id.clone()))],
                            0,
                            100,
                        )
                        .await?;

                    let message = format!(r#"Mục tiêu mới {} được gán cho bạn"#, new_obj.name);

                    for user in users {
                        notification_service
                            .create_noti(user.pk_user_id, message.clone(), vec![])
                            .await?;
                    }
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
