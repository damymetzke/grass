mod load;

use std::{
    cell::{Ref, RefCell},
    collections::hash_map::{Entry, HashMap},
    fs::{self, File},
    io::Read,
    path::PathBuf,
    rc::Rc,
};
use thiserror::Error;

use self::load::LoadRootConfig;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct GrassCategory {
    pub name: String,
    pub alias: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct GrassConfig {
    pub category: HashMap<String, Rc<RefCell<GrassCategory>>>,
    pub aliases: HashMap<String, Rc<RefCell<GrassCategory>>>,
    pub base_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct RootConfig {
    pub grass: GrassConfig,
}

impl GrassConfig {
    pub fn try_default() -> Option<Self> {
        Some(Self {
            category: HashMap::default(),
            aliases: HashMap::default(),
            base_dir: dirs::home_dir()?.join("repos"),
        })
    }
    pub fn get_from_category_or_alias<T>(&self, name: T) -> Option<Ref<GrassCategory>>
    where
        T: AsRef<str>,
    {
        self.category
            .get(name.as_ref())
            .or_else(|| self.aliases.get(name.as_ref()))
            .map(|value| value.borrow())
    }

    pub fn get_by_category<T>(&self, category_name: T) -> Option<Ref<GrassCategory>>
    where
        T: AsRef<str>,
    {
        self.category
            .get(category_name.as_ref())
            .map(|value| value.borrow())
    }

    pub fn get_by_alias<T>(&self, alias_name: T) -> Option<Ref<GrassCategory>>
    where
        T: AsRef<str>,
    {
        self.aliases
            .get(alias_name.as_ref())
            .map(|value| value.borrow())
    }
}

impl RootConfig {
    pub fn try_default() -> Option<Self> {
        Some(Self {
            grass: GrassConfig::try_default()?,
        })
    }

    // TODO: Return a Result
    pub fn merge(&mut self, next: &LoadRootConfig) -> &mut Self {
        let grass = if let Some(grass) = &next.grass {
            grass
        } else {
            return self;
        };

        if let Some(base_dir) = &grass.base_dir {
            match base_dir.strip_prefix("~/") {
                Some(suffix) => {
                    if let Some(home_dir) = dirs::home_dir() {
                        self.grass.base_dir = home_dir.join(suffix);
                    };
                }
                None => self.grass.base_dir = PathBuf::from(base_dir),
            }
        };

        for (key, category) in &grass.category {
            let category_rc = match self.grass.category.entry(key.clone()) {
                Entry::Vacant(e) => {
                    let result = Rc::from(RefCell::from(GrassCategory {
                        name: key.clone(),
                        alias: category.alias.clone(),
                    }));
                    e.insert(result).clone()
                }
                Entry::Occupied(e) => {
                    e.get().borrow_mut().name = key.clone();
                    e.get().clone()
                }
            };

            for alias in &category.alias {
                self.grass
                    .aliases
                    .insert(alias.clone(), category_rc.clone());
            }
        }

        self
    }
}

#[derive(Error, Debug)]
pub enum LoadUserError {
    #[error("Cannot find home directory")]
    MissingHomeDirectory,
    #[error("Cannot find configuration directory")]
    MissingConfigurationDirectory,
    #[error("Cannot read configuration file:\n{io_error}")]
    CannotReadConfigurationFile { io_error: std::io::Error },
    #[error("Cannot create default configuration")]
    CannotCreateDefault,
    #[error("The configuration file\n''\nwas improperly formatted:\n{reason}")]
    ImproperlyFormatted { file: PathBuf, reason: String },
}

// TODO: Convert to a proper library error
pub fn load_user_config() -> Result<RootConfig, LoadUserError> {
    let mut file = File::open(
        dirs::config_dir()
            .ok_or(LoadUserError::MissingHomeDirectory)?
            .join("grass/config.toml"),
    )
    .map_err(|error| LoadUserError::CannotReadConfigurationFile { io_error: error })?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|error| LoadUserError::CannotReadConfigurationFile { io_error: error })?;

    let mut config = RootConfig::try_default().ok_or(LoadUserError::CannotCreateDefault)?;

    let config_dir = dirs::config_dir()
        .ok_or(LoadUserError::MissingConfigurationDirectory)?
        .join("grass");
    if let Ok(entries) = fs::read_dir(&config_dir) {
        for file_name in
            entries
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| match entry.metadata() {
                    Ok(metadata) if metadata.is_file() => Some(entry.file_name()),
                    _ => None,
                })
        {
            // TODO: better handle these errors, this should not stop the program.
            let mut file = if let Ok(file) = File::open(&config_dir.join(&file_name)) {
                file
            } else {
                continue;
            };

            let mut contents = String::new();

            if file.read_to_string(&mut contents).is_err() {
                continue;
            };

            let load_config: LoadRootConfig =
                toml::from_str(&contents).map_err(|error| LoadUserError::ImproperlyFormatted {
                    file: PathBuf::from(file_name),
                    reason: error.to_string(),
                })?;
            config.merge(&load_config);
        }
    }

    Ok(config)
}

pub fn load_example_config() -> RootConfig {
    let general = Rc::from(RefCell::from(GrassCategory {
        name: String::from("general"),
        alias: vec![String::from("gen")],
    }));
    let work = Rc::from(RefCell::from(GrassCategory {
        name: String::from("work"),
        alias: Vec::new(),
    }));
    RootConfig {
        grass: GrassConfig {
            aliases: HashMap::from([(String::from("gen"), general.clone())]),
            category: HashMap::from([
                (String::from("general"), general),
                (String::from("work"), work),
            ]),
            base_dir: dirs::home_dir().unwrap().join("repos"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        load::{LoadGrassCategory, LoadGrassConfig},
        *,
    };

    fn get_load_config() -> LoadRootConfig {
        LoadRootConfig {
            grass: Some(LoadGrassConfig {
                category: HashMap::from([
                    (String::from("work"), LoadGrassCategory { alias: vec![] }),
                    (
                        String::from("general"),
                        LoadGrassCategory {
                            alias: vec![String::from("gen")],
                        },
                    ),
                ]),
                base_dir: Some(String::from("~/my-repositories")),
            }),
        }
    }

    #[test]
    fn test_config_merge() {
        let mut config = RootConfig::try_default().unwrap();
        config.merge(&get_load_config());

        // TODO: This tests depends on the existance of a home directory
        // Figure out a way that it doesn't.
        assert_eq!(
            config.grass.base_dir,
            dirs::home_dir().unwrap().join("my-repositories")
        );
        assert_eq!(
            config.grass.category.get("work").unwrap().borrow().name,
            "work"
        );
        assert_eq!(
            config.grass.category.get("general").unwrap().borrow().name,
            "general"
        );
        assert_eq!(
            config.grass.aliases.get("gen").unwrap().borrow().name,
            "general"
        );
    }

    #[test]
    fn test_config_get_category() {
        let config = load_example_config();

        let result_work = config.grass.get_by_category("work").unwrap();
        let result_general = config.grass.get_by_category("general").unwrap();

        assert_eq!(
            *result_work,
            GrassCategory {
                name: String::from("work"),
                alias: vec![]
            }
        );

        assert_eq!(
            *result_general,
            GrassCategory {
                name: String::from("general"),
                alias: vec![String::from("gen")]
            }
        );
    }

    #[test]
    fn test_config_get_alias() {
        let config = load_example_config();

        let result_gen = config.grass.get_by_alias("gen").unwrap();

        assert_eq!(
            *result_gen,
            GrassCategory {
                name: String::from("general"),
                alias: vec![String::from("gen")]
            }
        );
    }
}
