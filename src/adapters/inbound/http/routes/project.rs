use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
    adapters::inbound::http::handlers::create_project::{self, ProjectState},
    application::ports::inbound::create_project::CreateProjectUseCase,
};

pub(crate) fn project_router(create_project_use_case: Arc<dyn CreateProjectUseCase>) -> Router {
    let state = ProjectState::new(create_project_use_case);

    Router::new()
        .route("/projects", post(create_project::create_project))
        .with_state(state)
}
