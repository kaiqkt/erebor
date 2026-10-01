use std::sync::Arc;

use anyhow::{Error, Ok};
use async_trait::async_trait;
use sqlx::{Pool, Sqlite, prelude::FromRow};
use uuid::Uuid;

use crate::{
    application::ports::outbound::project_repository::ProjectRepository, domain::project::Project,
};

pub struct ProjectSqliteRepository {
    pool: Arc<Pool<Sqlite>>,
}

impl ProjectSqliteRepository {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }
}

#[derive(Debug, FromRow, PartialEq)]
#[allow(dead_code)]
struct ProjectTable {
    id: Uuid,
    name: String,
    description: String,
}

#[async_trait]
impl ProjectRepository for ProjectSqliteRepository {
    async fn save(&self, project: &Project) -> Result<(), Error> {
        sqlx::query("INSERT INTO projects (id, name, description) VALUES (?, ?, ?)")
            .bind(project.id)
            .bind(&project.name)
            .bind(&project.description)
            .execute(self.pool.as_ref())
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::{str::FromStr, sync::Arc};

    use crate::{
        adapters::outbound::persistence::project_sql_lite_repository::{
            ProjectSqliteRepository, ProjectTable,
        },
        application::ports::outbound::project_repository::ProjectRepository,
        domain::project::Project,
    };

    use sqlx::{
        Pool, Sqlite,
        sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    };

    #[tokio::test]
    async fn sqlite_save_successfully() {
        let pool = create_sqlite().await.unwrap();
        let repository = ProjectSqliteRepository::new(pool.clone());
        let project = Project::new(String::from("Erebor"), String::from("description"));

        repository.save(&project).await.unwrap();

        let saved_project = sqlx::query_as::<_, ProjectTable>(
            "SELECT id, name, description FROM projects WHERE id = ?",
        )
        .bind(project.id)
        .fetch_one(pool.as_ref())
        .await
        .unwrap();

        assert_eq!(saved_project.id, project.id);
        assert_eq!(saved_project.name, project.name);
        assert_eq!(saved_project.description, project.description);
    }

    async fn create_sqlite() -> anyhow::Result<Arc<Pool<Sqlite>>> {
        let options = SqliteConnectOptions::from_str("sqlite::memory:")?.create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;

        sqlx::migrate!("db/migrations").run(&pool).await?;

        Ok(Arc::new(pool))
    }
}
