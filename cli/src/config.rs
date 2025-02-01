use serde::{Deserialize, Serialize};
use std::fs::{self, create_dir_all, read_to_string};

use crate::logger;
use anyhow::Context;
use codesnap::config::{CodeSnap, SnapshotConfig};

const DEFAULT_CONFIG_CONTENT: &'static str = include_str!("../config.json");

#[derive(Serialize, Deserialize)]
pub struct CodeSnapCLIConfig {
    pub print_eggs: bool,
    pub snapshot_config: CodeSnap,
}

impl CodeSnapCLIConfig {
    pub fn from(content: &str) -> anyhow::Result<Self> {
        let config: Self = serde_json::from_str(&content)?;

        Ok(config)
    }
}

// Get CodeSnap config, create if the config does not exists
pub fn get_config_content() -> anyhow::Result<String> {
    let home_dir = home::home_dir().context("Unable to get your home dir")?;
    let codesnap_home_path = home_dir.join(".config").join("CodeSnap");
    let config_path = codesnap_home_path.join("config.json");
    let is_config_exists = config_path.try_exists()?;

    let content = if is_config_exists {
        read_to_string(&config_path)
    } else {
        create_dir_all(&codesnap_home_path)?;
        fs::write(&config_path, DEFAULT_CONFIG_CONTENT)?;

        logger::info(&format!(
            "Automated created config file at {:?}",
            &config_path
        ));

        Ok(DEFAULT_CONFIG_CONTENT.to_string())
    }?;

    Ok(content)
}
