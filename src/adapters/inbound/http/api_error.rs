use std::collections::HashMap;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

use crate::domain::domain_error::DomainError;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    Validation(#[from] ValidationErrors),

    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("unexpected error")]
    Unexpected,
}

#[derive(Debug, Serialize)]
struct ApiErrorResponse<T> {
    code: &'static str,
    details: Option<T>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::Validation(errors) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiErrorResponse {
                    code: "invalid_request",
                    details: Some(handle_errors(errors)),
                }),
            )
                .into_response(),
            Self::Domain(error) => {
                let (status, code) = match &error {
                    DomainError::ProjectNotFound { .. } => {
                        (StatusCode::NOT_FOUND, "project_not_found")
                    }
                };

                (
                    status,
                    Json(ApiErrorResponse {
                        code,
                        details: Some(code),
                    }),
                )
                    .into_response()
            }
            Self::Unexpected => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::<String> {
                    code: "internal_server_error",
                    details: None,
                }),
            )
                .into_response(),
        }
    }
}

fn handle_errors(validation_errors: ValidationErrors) -> HashMap<String, Vec<String>> {
    validation_errors
        .field_errors()
        .into_iter()
        .map(|(field, errors)| {
            let messages: Vec<String> = errors
                .iter()
                .map(|error| {
                    error
                        .message
                        .as_deref()
                        .unwrap_or(error.code.as_ref())
                        .to_string()
                })
                .collect();

            (field.to_string(), messages)
        })
        .collect()
}
