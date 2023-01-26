use crate::domain::{
    model::application::{Application, NewApplication},
    repository::application_repository::ApplicationRepository,
};

use anyhow::Result;

#[derive(Default, Clone)]
pub struct ApplicationMiddleware<ApplicationRepository> {
    repository: ApplicationRepository,
}

impl ApplicationMiddleware<ApplicationRepository> {
    pub fn new(repository: ApplicationRepository) -> Self {
        Self { repository }
    }

    pub fn create(&self, new_application: NewApplication) -> Result<Application> {
        let application = self.repository.insert(new_application)?;

        Ok(application)
    }
}
