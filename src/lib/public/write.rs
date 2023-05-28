use git2::build::RepoBuilder;

use crate::{config::RootConfig, facades::git};

pub fn clone_repository<T, U>(user_config: &RootConfig, remote: T, category: U) -> Option<()>
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    let base_dir = crate::dev::get_category_path(user_config, category)?;
    let repository_name = remote
        .as_ref()
        .split('/')
        .last()
        .unwrap_or("repository")
        .trim_end_matches(".git");

    let repo_path = base_dir.join(repository_name);

    RepoBuilder::new().clone(remote.as_ref(), &repo_path).ok()?;

    Some(())
}

pub fn clean_repository<T, U>(user_config: &RootConfig, category: T, repository: U) -> Option<()>
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    let repository_path = crate::dev::get_repository_path(user_config, category, repository)?;
    // TODO: Handle errors
    git::clean_repository(repository_path).ok()
}
