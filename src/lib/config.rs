mod load;

use std::{
    cell::{Ref, RefCell},
    collections::hash_map::{Entry, HashMap},
    fs::{self, File},
    io::Read,
    rc::Rc,
};

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
    pub base_dir: String,
}

impl Default for GrassConfig {
    fn default() -> Self {
        GrassConfig {
            category: HashMap::default(),
            aliases: HashMap::default(),
            base_dir: String::from("~/.repos"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RootConfig {
    pub grass: GrassConfig,
}

impl GrassConfig {
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
    pub fn merge(&mut self, next: &LoadRootConfig) -> &mut Self {
        let grass = if let Some(grass) = &next.grass {
            grass
        } else {
            return self;
        };

        if let Some(base_dir) = &grass.base_dir {
            self.grass.base_dir = base_dir.clone();
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

pub fn load_user_config() -> Result<RootConfig, Box<dyn std::error::Error>> {
    let mut file = File::open(
        dirs::config_dir()
            .ok_or("Could not find home directory")?
            .join("grass/config.toml"),
    )?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut config = RootConfig::default();
    let load_config: LoadRootConfig = toml::from_str(&contents)?;
    config.merge(&load_config);

    let mut config = RootConfig::default();

    let config_dir = dirs::config_dir()
        .ok_or("Could not find config directory")?
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
            let mut file = File::open(&config_dir.join(file_name))?;

            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let load_config: LoadRootConfig = toml::from_str(&contents)?;
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
            base_dir: String::from("~/repos"),
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
        let mut config = RootConfig::default();
        config.merge(&get_load_config());

        assert_eq!(config.grass.base_dir, "~/my-repositories");
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
