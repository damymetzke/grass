use std::{collections::hash_map::HashMap, fs::File, io::Read};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct GrassCategory {}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct GrassConfig {
    pub category: HashMap<String, GrassCategory>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct RootConfig {
    #[serde(default)]
    pub grass: GrassConfig,
}

pub fn load_user_config() -> Result<RootConfig, Box<dyn std::error::Error>> {
    let mut file = File::open(
        dirs::config_dir()
            .ok_or("Could not find home directory")?
            .join("grass/config.toml"),
    )?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: RootConfig = toml::from_str(&contents)?;
    Ok(config)
}
