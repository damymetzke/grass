use git2::{build::RepoBuilder, Repository};

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

pub fn clean_repository<T, U>(user_config: &RootConfig, category: T, repository: U) -> Option<()>
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    let repository_path = crate::dev::get_repository_path(user_config, category, repository)?;
    let repo = match Repository::open(&repository_path) {
        Ok(repo) => repo,
        Err(_) => return None,
    };

    let statuses = match repo.statuses(None) {
        Ok(statuses) => statuses,
        Err(_) => return None,
    };

    let files_to_remove = statuses.iter().filter_map(|entry| match entry.path() {
        Some(file_path) if entry.status().is_ignored() => Some(file_path.to_owned()),
        _ => None,
    });

    for file in files_to_remove {
        let path = repository_path.join(&file);
        if path.is_dir() {
            if std::fs::remove_dir_all(path).is_err() {
                return None;
            }
        } else if std::fs::remove_file(path).is_err() {
            return None;
        }
    }

    Some(())
}
