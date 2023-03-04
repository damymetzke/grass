mod load;

use std::{
    collections::hash_map::{Entry, HashMap},
    fs::{self, File},
    io::Read,
};

use self::load::LoadRootConfig;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct GrassCategory {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GrassConfig {
    pub category: HashMap<String, GrassCategory>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RootConfig {
    pub grass: GrassConfig,
}

impl RootConfig {
    pub fn merge(&mut self, next: LoadRootConfig) -> &mut Self {
        let grass = if let Some(grass) = next.grass {
            grass
        } else {
            return self;
        };

        for (key, _) in grass.category {
            match self.grass.category.entry(key.clone()) {
                Entry::Vacant(e) => {
                    e.insert(GrassCategory { name: key });
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().name = key;
                }
            };
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
    config.merge(load_config);

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
            config.merge(load_config);
        }
    }

    Ok(config)
}

pub fn load_example_config() -> RootConfig {
    RootConfig {
        grass: GrassConfig {
            category: HashMap::from([
                (
                    String::from("general"),
                    GrassCategory {
                        name: String::from("general"),
                    },
                ),
                (
                    String::from("work"),
                    GrassCategory {
                        name: String::from("work"),
                    },
                ),
            ]),
        },
    }
}
