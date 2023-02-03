use crate::domain::{
    dto::__MODULE_NAME__::{
        New__DATA_CLASS_STRUCT_NAME__InputDTO, Update__DATA_CLASS_STRUCT_NAME__InputDTO,
        __DATA_CLASS_STRUCT_NAME__DetailsDTO, __DATA_CLASS_STRUCT_NAME__ListItemDTO,
    },
    model::user::User,
    model::__MODULE_NAME__::{__DATA_CLASS_STRUCT_NAME__, __NEW_DATA_CLASS_STRUCT_NAME__},
    repository::__MODULE_NAME___repository::__DATA_CLASS_STRUCT_NAME__Repository,
};
use anyhow::Result;
use chrono::Utc;

#[derive(Clone)]
pub struct __DATA_CLASS_STRUCT_NAME__Middleware {
    repository: __DATA_CLASS_STRUCT_NAME__Repository,
}

impl __DATA_CLASS_STRUCT_NAME__Middleware {
    pub fn new(repository: __DATA_CLASS_STRUCT_NAME__Repository) -> Self {
        Self { repository }
    }

    pub fn find_one_by_id(
        &self,
        __MODULE_NAME___id: &str,
    ) -> Result<Option<__DATA_CLASS_STRUCT_NAME__>> {
        let __MODULE_NAME___real_id = __MODULE_NAME___id.parse::<i32>()?;

        let __MODULE_NAME___found = self.repository.find_one_by_id(__MODULE_NAME___real_id)?;

        Ok(__MODULE_NAME___found)
    }

    pub fn find_for_user(
        &self,
        user: &User,
        page: i32,
        per_page: i32,
    ) -> anyhow::Result<Vec<__DATA_CLASS_STRUCT_NAME__>> {
        let __MODULE_NAME__s =
            self.repository
                .find_all_for_application_id(user.application_id, page, per_page)?;

        Ok(__MODULE_NAME__s)
    }

    pub fn find_one_for_user(
        &self,
        __MODULE_NAME___id: &str,
        user: &User,
    ) -> anyhow::Result<Option<__DATA_CLASS_STRUCT_NAME__>> {
        let __MODULE_NAME___real_id = __MODULE_NAME___id.parse::<i32>()?;

        let __MODULE_NAME__ = self
            .repository
            .find_one_with_application_id(__MODULE_NAME___real_id, user.application_id)?;

        Ok(__MODULE_NAME__)
    }

    pub fn create_from_user_input(
        &self,
        creator: &User,
        dto: New__DATA_CLASS_STRUCT_NAME__InputDTO,
    ) -> Result<__DATA_CLASS_STRUCT_NAME__> {
        let new__MODULE_NAME__ = New__DATA_CLASS_STRUCT_NAME__ {
            // ...
            application_id: creator.application_id,
            created_date: Utc::now(),
            created_by: Some(creator.id),
            deleted_date: None,
            deleted_by: None,
            is_deleted: false,
        };

        self.create(new__MODULE_NAME__)
    }

    pub fn update_from_user_input(
        &self,
        updater: &User,
        to_update: &__DATA_CLASS_STRUCT_NAME__,
        dto: Update__DATA_CLASS_STRUCT_NAME__InputDTO,
    ) -> Result<__DATA_CLASS_STRUCT_NAME__> {
        let mut __MODULE_NAME__ = to_update.clone();

        // ...

        self.update(&__MODULE_NAME__)
    }

    pub fn create(
        &self,
        new___MODULE_NAME__: New__DATA_CLASS_STRUCT_NAME__,
    ) -> Result<__DATA_CLASS_STRUCT_NAME__> {
        let mut new___MODULE_NAME__ = new___MODULE_NAME__.clone();

        let __MODULE_NAME__ = self.repository.insert(new___MODULE_NAME__)?;

        Ok(__MODULE_NAME__)
    }

    pub fn update(
        &self,
        __MODULE_NAME__: &__DATA_CLASS_STRUCT_NAME__,
    ) -> Result<__DATA_CLASS_STRUCT_NAME__> {
        self.repository.update(__MODULE_NAME__)
    }

    pub fn delete(
        &self,
        __MODULE_NAME___to_delete: &__DATA_CLASS_STRUCT_NAME__,
        deleter: &User,
    ) -> Result<bool> {
        let mut cloned = __MODULE_NAME___to_delete.clone();

        cloned.deleted_date = Some(Utc::now());
        cloned.deleted_by = Some(deleter.id);
        cloned.is_deleted = true;

        self.update(&cloned)?;

        Ok(true)
    }

    pub fn to_list_dto(
        &self,
        __MODULE_NAME__s: Vec<__DATA_CLASS_STRUCT_NAME__>,
    ) -> Vec<__DATA_CLASS_STRUCT_NAME__ListItemDTO> {
        let mut list = Vec::<__DATA_CLASS_STRUCT_NAME__ListItemDTO>::new();

        for __MODULE_NAME__ in __MODULE_NAME__s.iter() {
            list.push(__DATA_CLASS_STRUCT_NAME__ListItemDTO::from(__MODULE_NAME__));
        }

        list
    }

    pub fn to_details_dto(
        &self,
        __MODULE_NAME__: &__DATA_CLASS_STRUCT_NAME__,
    ) -> __DATA_CLASS_STRUCT_NAME__DetailsDTO {
        __DATA_CLASS_STRUCT_NAME__DetailsDTO::from(__MODULE_NAME__)
    }
}
