use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::{
        dto::create_project::CreateProjectDto,
        error::ApplicationError,
        ports::{
            inbound::create_project::CreateProjectUseCase,
            outbound::project_repository::ProjectRepository,
        },
    },
    domain::project::Project,
};

pub struct CreateProjectService {
    repository: Arc<dyn ProjectRepository + Send + Sync>,
}

impl CreateProjectService {
    pub fn new(repository: Arc<dyn ProjectRepository + Send + Sync>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CreateProjectUseCase for CreateProjectService {
    async fn create(&self, dto: CreateProjectDto) -> Result<Project, ApplicationError> {
        let project = Project::new(dto.name, dto.description);

        self.repository
            .save(&project)
            .await
            .map_err(ApplicationError::Unexpected)?;

        Ok(project)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;

    use crate::{
        application::{
            dto::create_project::CreateProjectDto,
            ports::{
                inbound::create_project::CreateProjectUseCase,
                outbound::project_repository::ProjectRepository,
            },
            use_cases::create_project::CreateProjectService,
        },
        domain::project::Project,
    };

    #[derive(Clone, Default)]

    struct MockProjectRepository {
        received_project: Arc<Mutex<Option<Project>>>,
    }

    #[async_trait]
    impl ProjectRepository for MockProjectRepository {
        async fn save(&self, project: &Project) -> Result<(), anyhow::Error> {
            *self.received_project.lock().unwrap() = Some(project.clone());

            Ok(())
        }
    }

    #[tokio::test]
    async fn create_project_successfully() {
        let mock = MockProjectRepository::default();
        let received_project = Arc::clone(&mock.received_project);
        let service = CreateProjectService::new(Arc::new(mock));

        let project = service
            .create(CreateProjectDto {
                name: String::from("Erebor"),
                description: String::from("description"),
            })
            .await
            .unwrap();

        let saved_project = received_project.lock().unwrap().clone().unwrap();
        assert_eq!(project.id(), saved_project.id());
        assert_eq!(project.name(), saved_project.name());
        assert_eq!(project.description(), saved_project.description())
    }
}
