use std::sync::Arc;

use crate::{
    department::service::DepartmentService, key_result::service::KeyResultService,
    objectives::service::ObjectiveService, periods::service::PeriodService, prisma::PrismaClient,
    users::service::UserService,
};

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub obj_service: ObjectiveService,
    pub department_service: DepartmentService,
    pub keyresult_service: KeyResultService,
    pub period_service: PeriodService,
}

impl AppState {
    pub async fn new(client: Arc<PrismaClient>) -> Self {
        Self {
            user_service: UserService::init(&client),
            obj_service: ObjectiveService::init(&client),
            department_service: DepartmentService::init(&client),
            keyresult_service: KeyResultService::init(&client),
            period_service: PeriodService::init(&client),
        }
    }
}
