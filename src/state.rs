use crate::{
    aws::S3, comment::service::CommentService, department::service::DepartmentService,
    file::service::FileService, file_version::service::FileVersionService,
    folder::service::FolderService, key_result::service::KeyResultService,
    notification::service::NotificationService, objectives::service::ObjectiveService,
    organize::service::OrganizeService, periods::service::PeriodService, prisma::PrismaClient,
    tag::service::TagService, users::service::UserService,
};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::sync::broadcast;

use std::collections::{HashMap, HashSet};

pub struct RoomState {
    pub users: Mutex<HashSet<String>>,
    pub tx: broadcast::Sender<String>,
}

impl RoomState {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashSet::new()),
            tx: broadcast::channel(69).0,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub obj_service: ObjectiveService,
    pub department_service: DepartmentService,
    pub notification_service: NotificationService,
    pub keyresult_service: KeyResultService,
    pub period_service: PeriodService,
    pub organize_service: OrganizeService,
    pub folder_service: FolderService,
    pub file_service: FileService,
    pub file_version_service: FileVersionService,
    pub tag_service: TagService,
    pub storage: S3,
    pub comment_service: CommentService,

    pub rooms: Arc<Mutex<HashMap<String, RoomState>>>,
}

impl AppState {
    pub async fn new(client: Arc<PrismaClient>) -> Self {
        Self {
            user_service: UserService::init(&client),
            obj_service: ObjectiveService::init(&client),
            department_service: DepartmentService::init(&client),
            notification_service: NotificationService::init(&client),
            keyresult_service: KeyResultService::init(&client),
            period_service: PeriodService::init(&client),
            organize_service: OrganizeService::init(&client),
            folder_service: FolderService::init(&client),
            file_service: FileService::init(&client),
            file_version_service: FileVersionService::init(&client),
            tag_service: TagService::init(&client),
            comment_service: CommentService::init().await,
            storage: S3::init(),
            rooms: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
