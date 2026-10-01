use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Project {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) description: String,
}

impl Project {
    pub(crate) fn new(name: String, description: String) -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            name,
            description,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateProjectDto {
    pub(crate) name: String,
    pub(crate) description: String,
}
