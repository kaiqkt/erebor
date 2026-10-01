use thiserror::Error;

use crate::domain::domain_error::DomainError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("unexpected application error")]
    Unexpected(#[source] anyhow::Error),
}
