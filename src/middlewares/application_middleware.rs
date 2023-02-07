use crate::domain::{
    dto::application::{ApplicationDetailsDTO, ApplicationListItemDTO},
    model::{
        application::{Application, NewApplication},
        user::User,
    },
    repository::application_repository::ApplicationRepository,
};

use anyhow::Result;

/// Application model middleware.
#[derive(Clone)]
pub struct ApplicationMiddleware {
    repository: ApplicationRepository,
}

impl ApplicationMiddleware {
    /// constructor
    pub fn new(repository: ApplicationRepository) -> Self {
        Self { repository }
    }

    /// creates (e.g. inserts) an Application into the database.
    pub fn create(&self, new_application: NewApplication) -> Result<Application> {
        let application = self.repository.insert(new_application)?;

        Ok(application)
    }

    /// finds an application by its ID
    pub fn find_one_by_id(&self, id: &str) -> Result<Option<Application>> {
        let real_id = id.parse::<i32>()?;

        let account = self.repository.find_one_by_id(real_id)?;

        Ok(account)
    }

    /// finds all applications for a given user.
    /// this function uses pagination.
    pub fn find_for_user(&self, user: &User, page: u16, per_page: u16) -> Result<Vec<Application>> {
        let applications = self
            .repository
            .find_all_for_user(user, page.into(), per_page.into())?;

        Ok(applications)
    }

    /// find a given application by its ID, for a given user.
    pub fn find_one_for_user(&self, id: &str, user: &User) -> Result<Option<Application>> {
        let real_id = id.parse::<i32>()?;

        let application = self.repository.find_for_user(real_id, user)?;

        Ok(application)
    }

    /// transforms a list of Application into a list of ApplicationListItemDTO
    pub fn to_list_dto(&self, applications: Vec<Application>) -> Vec<ApplicationListItemDTO> {
        let mut list = Vec::<ApplicationListItemDTO>::new();

        for application in applications.iter() {
            list.push(ApplicationListItemDTO::from(application));
        }

        list
    }

    /// transforms an Application into a ApplicationDetailsDTO
    pub fn to_details_dto(&self, application: &Application) -> ApplicationDetailsDTO {
        ApplicationDetailsDTO::from(application)
    }
}
