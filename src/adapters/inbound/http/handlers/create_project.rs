use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    adapters::inbound::http::api_error::ApiError,
    application::{
        dto::create_project::CreateProjectDto, ports::inbound::create_project::CreateProjectUseCase,
    },
    domain::domain_error::DomainError,
};

#[derive(Clone)]
pub(crate) struct ProjectState {
    create_project: Arc<dyn CreateProjectUseCase>,
}

impl ProjectState {
    pub(crate) fn new(create_project: Arc<dyn CreateProjectUseCase>) -> Self {
        Self { create_project }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProjectRequest {
    #[validate(length(
        min = 1,
        max = 50,
        message = "name must contain betwaeen 1 and 50 characters"
    ))]
    name: String,
    #[validate(length(max = 255, message = "description must not exceed 255 characters"))]
    description: String,
}

impl From<CreateProjectRequest> for crate::application::dto::create_project::CreateProjectDto {
    fn from(project: CreateProjectRequest) -> Self {
        Self {
            name: project.name.to_owned().trim().to_string(),
            description: project.description.to_owned().trim().to_string(),
        }
    }
}

#[derive(Serialize)]
struct CreateProjectResponse {
    id: Uuid,
    name: String,
    description: String,
}

impl From<crate::domain::project::Project> for CreateProjectResponse {
    fn from(project: crate::domain::project::Project) -> Self {
        Self {
            id: project.id(),
            name: project.name().to_owned(),
            description: project.description().to_owned(),
        }
    }
}

pub async fn create_project(
    State(state): State<ProjectState>,
    Json(body): Json<CreateProjectRequest>,
) -> Result<impl IntoResponse, ApiError> {
    body.validate()?;

    let dto = CreateProjectDto::from(body);
    let project = state.create_project.create(dto).await.map_err(|error| {
        error
            .downcast::<DomainError>()
            .map(ApiError::Domain)
            .unwrap_or(ApiError::Unexpected)
    })?;

    Ok((
        StatusCode::CREATED,
        Json(CreateProjectResponse::from(project)),
    ))
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use anyhow::Error;
    use async_trait::async_trait;
    use axum::{body::to_bytes, response::IntoResponse};
    use serde_json::Value;

    use super::*;
    use crate::{
        application::{
            dto::create_project::CreateProjectDto,
            ports::inbound::create_project::CreateProjectUseCase,
        },
        domain::project::Project,
    };

    #[derive(Clone, Default)]
    struct MockCreateProjectUseCase {
        received_dto: Arc<Mutex<Option<CreateProjectDto>>>,
    }

    #[async_trait]
    impl CreateProjectUseCase for MockCreateProjectUseCase {
        async fn create(&self, dto: CreateProjectDto) -> Result<Project, Error> {
            *self.received_dto.lock().unwrap() = Some(dto.clone());

            Ok(Project::new(dto.name, dto.description))
        }
    }

    #[tokio::test]
    async fn creates_project_and_trims_request_fields() {
        let mock = MockCreateProjectUseCase::default();
        let received_dto = Arc::clone(&mock.received_dto);
        let state = ProjectState::new(Arc::new(mock));

        let response = create_project(
            State(state),
            Json(CreateProjectRequest {
                name: "  Erebor  ".to_owned(),
                description: "  projeto de exemplo  ".to_owned(),
            }),
        )
        .await
        .unwrap()
        .into_response();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["name"], "Erebor");
        assert_eq!(json["description"], "projeto de exemplo");
        assert!(json["id"].as_str().is_some());

        let dto = received_dto.lock().unwrap().clone().unwrap();
        assert_eq!(dto.name, "Erebor");
        assert_eq!(dto.description, "projeto de exemplo");
    }

    #[tokio::test]
    async fn rejects_request_with_empty_name() {
        let mock = MockCreateProjectUseCase::default();
        let received_dto = Arc::clone(&mock.received_dto);
        let state = ProjectState::new(Arc::new(mock));

        let result = create_project(
            State(state),
            Json(CreateProjectRequest {
                name: "".to_owned(),
                description: "descrição".to_owned(),
            }),
        )
        .await;

        let error = match result {
            Ok(_) => panic!("request with an empty name should fail"),
            Err(error) => error,
        };

        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert!(received_dto.lock().unwrap().is_none());
    }
}
