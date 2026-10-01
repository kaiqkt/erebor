use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

use anyhow::Ok;

use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::{
    adapters::{
        inbound::http::routes::{common::router, project::project_router},
        outbound::persistence::project_sql_lite_repository::ProjectSqliteRepository,
    },
    app::App,
    application::use_cases::create_project::CreateProjectService,
};

pub(crate) async fn build() -> anyhow::Result<App> {
    let sqlite = create_sqlite().await?;

    let project_repository = Arc::new(ProjectSqliteRepository::new(sqlite));
    let create_project_service = Arc::new(CreateProjectService::new(project_repository));

    let router = router();
    let project_router = project_router(create_project_service).merge(router);

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    Ok(App::new(project_router, socket))
}

async fn create_sqlite() -> anyhow::Result<Arc<Pool<Sqlite>>> {
    let options = SqliteConnectOptions::from_str("sqlite://database.db")?.create_if_missing(true);

    let pool = SqlitePoolOptions::new().connect_with(options).await?;

    sqlx::migrate!("db/migrations").run(&pool).await?;

    Ok(Arc::new(pool))
}
