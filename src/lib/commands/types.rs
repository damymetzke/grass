pub struct SimpleCategoryDescription {
    pub category: String,
    pub repositories: Vec<String>,
}

pub struct SimpleRepositoryDescription {
    pub category: String,
    pub repository: String,
}

impl SimpleRepositoryDescription {
    pub fn to_session_string(&self) -> String {
        format!("{}@{}", self.repository, self.category)
    }
}
