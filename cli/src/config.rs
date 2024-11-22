use std::fs::{self, create_dir_all, read_to_string};

use crate::logger;
use anyhow::Context;

const DEFAULT_CONFIG_CONTENT: &'static str = r##"{
  "window": {
    "macWindowBar": true,
    "shadow": 20,
    "margin": {
      "x": 82,
      "y": 82
    }
  },
  "code": {
    "fontFamily": "CaskaydiaCove Nerd Font",
    "theme": "candy"
  },
  "watermark": {
    "content": "CodeSnap",
    "fontFamily": "Pacifico",
    "color": "#ffffff"
  },
  "background":  {
    "start": {
      "x": 0,
      "y": 0
    },
    "end": {
      "x": "max",
      "y": 0
    },
    "stops": [
      {
        "position": 0,
        "color": "#6bcba5"
      },
      {
        "position": 1,
        "color": "#caf4c2"
      }
    ]
  }
}"##;

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
