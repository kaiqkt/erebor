use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Project {
    id: Uuid,
    name: String,
    description: String,
}

impl Project {
    pub fn new(name: String, description: String) -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            name,
            description,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
