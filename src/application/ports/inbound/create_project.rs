use anyhow::Error;
use async_trait::async_trait;

use crate::{application::dto::create_project::CreateProjectDto, domain::project::Project};

#[async_trait]
pub(crate) trait CreateProjectUseCase: Send + Sync + 'static {
    async fn create(&self, dto: CreateProjectDto) -> Result<Project, Error>;
}
