use git2::build::RepoBuilder;

use crate::config::RootConfig;

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
