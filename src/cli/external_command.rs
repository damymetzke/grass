use std::{env, fs, os::unix::prelude::PermissionsExt, path::PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub struct ExternalCommand {
    pub path: PathBuf,
    pub command: String,
}

// I've generated this code using AI.
// I have verified it myself for any problems, and haven't found any.
pub fn get_external_commands() -> Vec<ExternalCommand> {
    let paths = env::var_os("PATH").unwrap_or_default();
    let paths: Vec<PathBuf> = env::split_paths(&paths).collect();

    let mut commands = Vec::new();

    for path in paths {
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    if let Some(file_name) = file_name.to_str() {
                        if file_name.starts_with("grass-") {
                            let command = file_name.strip_prefix("grass-").unwrap().to_owned();
                            let file_path = entry.path();

                            if file_path.is_file()
                                && file_path
                                    .metadata()
                                    .map(|m| m.permissions().mode() & 0o111 != 0)
                                    .unwrap_or(false)
                            {
                                commands.push(ExternalCommand {
                                    path: file_path,
                                    command,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    commands.dedup();
    commands
}
