#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct RepositoryLocation {
    pub category: String,
    pub repository: String,
}

impl<T, U> From<(T, U)> for RepositoryLocation
where
    T: Into<String>,
    U: Into<String>,
{
    fn from((category, repository): (T, U)) -> Self {
        Self {
            category: category.into(),
            repository: repository.into(),
        }
    }
}

impl<T, U> From<&(T, U)> for RepositoryLocation
where
    T: Into<String> + Clone,
    U: Into<String> + Clone,
{
    fn from((category, repository): &(T, U)) -> Self {
        Self {
            category: category.clone().into(),
            repository: repository.clone().into(),
        }
    }
}
