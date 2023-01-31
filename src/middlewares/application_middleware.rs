use crate::domain::{
    dto::application::{ApplicationDetailsDTO, ApplicationListItemDTO},
    model::{
        application::{Application, NewApplication},
        user::User,
    },
    repository::application_repository::ApplicationRepository,
};

use anyhow::Result;

#[derive(Clone)]
pub struct ApplicationMiddleware {
    repository: ApplicationRepository,
}

impl ApplicationMiddleware {
    pub fn new(repository: ApplicationRepository) -> Self {
        Self { repository }
    }

    pub fn create(&self, new_application: NewApplication) -> Result<Application> {
        let application = self.repository.insert(new_application)?;

        Ok(application)
    }

    pub fn find_for_user(&self, user: &User) -> Result<Option<Application>> {
        let application = self.repository.find_one_for_user(user)?;

        Ok(application)
    }

    pub fn to_list_dto(&self, applications: Vec<Application>) -> Vec<ApplicationListItemDTO> {
        let mut list = Vec::<ApplicationListItemDTO>::new();

        for application in applications.iter() {
            list.push(ApplicationListItemDTO::from(application));
        }

        list
    }

    pub fn to_details_dto(&self, application: &Application) -> ApplicationDetailsDTO {
        ApplicationDetailsDTO::from(application)
    }
}
