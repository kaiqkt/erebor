#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateProjectDto {
    pub(crate) name: String,
    pub(crate) description: String,
}
