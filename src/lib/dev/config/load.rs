use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct LoadGrassCategory {
    pub alias: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct LoadGrassConfig {
    pub category: HashMap<String, LoadGrassCategory>,
    pub base_dir: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct LoadRootConfig {
    pub grass: Option<LoadGrassConfig>,
}
