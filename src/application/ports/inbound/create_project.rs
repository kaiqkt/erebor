use anyhow::Error;
use async_trait::async_trait;

use crate::domain::project::{CreateProjectDto, Project};

#[async_trait]
pub(crate) trait CreateProjectUseCase: Send + Sync + 'static {
    async fn create(&self, dto: CreateProjectDto) -> Result<Project, Error>;
}
