use anyhow::Error;
use async_trait::async_trait;

use crate::domain::project::Project;

#[async_trait]
pub(crate) trait ProjectRepository: Send + Sync + 'static {
    async fn save(&self, project: &Project) -> Result<(), Error>;
}
