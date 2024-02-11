use std::sync::Arc;

use crate::{
    department::service::DepartmentService, objectives::service::ObjectiveService,
    prisma::PrismaClient, users::service::UserService,
};

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub obj_service: ObjectiveService,
    pub department_service: DepartmentService,
}

impl AppState {
    pub async fn new(client: Arc<PrismaClient>) -> Self {
        Self {
            user_service: UserService::init(&client),
            obj_service: ObjectiveService::init(&client),
            department_service: DepartmentService::init(&client),
        }
    }
}
