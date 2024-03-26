use crate::{
    error::ErrorResponse,
    folder::model::select::{folder_select, FolderSelect},
    prisma::{folder, tag, user, Visibility},
    tag::model::select::Tag,
    users::model::response::UserSelect,
};

use super::FolderService;

impl FolderService {
    pub async fn update_folder(
        &self,
        folder_id: String,
        parent: Option<String>,
        folder_name: Option<String>,
        visibility: Option<Visibility>,
    ) -> Result<FolderSelect, ErrorResponse> {
        let mut changes = vec![];

        if let Some(parent) = parent {
            changes.push(folder::parent_folder_id::set(Some(parent)))
        }

        if let Some(folder_name) = folder_name {
            changes.push(folder::folder_name::set(folder_name))
        }

        if let Some(visibility) = visibility {
            changes.push(folder::visibility::set(visibility))
        }

        let updated_folder = self
            .db
            .folder()
            .update_unchecked(folder::id::equals(folder_id), changes)
            .select(folder_select::select())
            .exec()
            .await?;
        Ok(updated_folder)
    }

    pub async fn set_collaborators_to_folder(
        &self,
        folder_id: String,
        collaborators: Vec<UserSelect>,
    ) -> Result<(), ErrorResponse> {
        let collaborators = collaborators
            .into_iter()
            .map(|collaborator| user::pk_user_id::equals(collaborator.pk_user_id))
            .collect();

        self.db
            .folder()
            .update(
                folder::id::equals(folder_id),
                vec![folder::collaborators::set(collaborators)],
            )
            .exec()
            .await?;
        Ok(())
    }

    pub async fn set_tags_to_folder(
        &self,
        tags: Vec<Tag>,
        folder_id: String,
    ) -> Result<(), ErrorResponse> {
        let tags = tags
            .into_iter()
            .map(|tag| tag::id::equals(tag.id))
            .collect();

        self.db
            .folder()
            .update(folder::id::equals(folder_id), vec![folder::tags::set(tags)])
            .exec()
            .await?;
        Ok(())
    }
}
