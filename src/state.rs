use std::sync::Arc;

use crate::{
    aws::S3, department::service::DepartmentService, file::service::FileService,
    file_version::service::FileVersionService, key_result::service::KeyResultService,
    objectives::service::ObjectiveService, organize::service::OrganizeService,
    periods::service::PeriodService, prisma::PrismaClient, tag::service::TagService,
    users::service::UserService,
};

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub obj_service: ObjectiveService,
    pub department_service: DepartmentService,
    pub keyresult_service: KeyResultService,
    pub period_service: PeriodService,
    pub organize_service: OrganizeService,
    pub folder_service: FolderService,
    pub file_service: FileService,
    pub file_version_service: FileVersionService,
    pub tag_service: TagService,
    pub storage: S3,
}

impl AppState {
    pub async fn new(client: Arc<PrismaClient>) -> Self {
        Self {
            user_service: UserService::init(&client),
            obj_service: ObjectiveService::init(&client),
            department_service: DepartmentService::init(&client),
            keyresult_service: KeyResultService::init(&client),
            period_service: PeriodService::init(&client),
            organize_service: OrganizeService::init(&client),
            folder_service: FolderService::init(&client),
            file_service: FileService::init(&client),
            file_version_service: FileVersionService::init(&client),
            tag_service: TagService::init(&client),
            storage: S3::init(),
        }
    }
}
