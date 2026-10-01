use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("project with id `{id}` was not found")]
    ProjectNotFound { id: Uuid },
}
