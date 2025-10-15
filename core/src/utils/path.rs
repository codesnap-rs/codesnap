use chrono::Local;
use regex::Regex;
use std::{
    env::{var, VarError},
    path::{Path, PathBuf},
};

// If users provide a directory path, we will generate a temporary file name
// to save the screenshot.
// The generation rule of the file name follows the below pattern:
//
// CodeSnap_y-m-d_at_h:m:s
//
fn create_temp_file_name() -> String {
    let now = Local::now();
    let formatted_time = now.format("%Y-%m-%d_at_%H:%M:%S");

    format!("CodeSnap_{}", formatted_time)
}

pub fn parse_home_variable(path: &str) -> Result<String, VarError> {
    if cfg!(windows) {
        return Ok(path.to_string());
    }

    let home_path = var("HOME")?;
    let regex = Regex::new(r"(~|$HOME)").unwrap();
    let path = regex.replace_all(path, home_path);

    Ok(path.to_string())
}

pub fn parse_file_name(path: &str) -> Result<String, VarError> {
    let path_str = parse_home_variable(path)?;
    let path = Path::new(&path_str);
    let parsed_path = if path.is_dir() {
        path.join(create_temp_file_name())
            .to_str()
            .unwrap()
            .to_string()
    } else {
        path_str
    };

    Ok(parsed_path)
}

pub fn get_config_home_path() -> Result<PathBuf, VarError> {
    let home = parse_home_variable("~")?;
    let config_home_path = Path::new(&home).join(".config").join("codesnap");

    if !config_home_path.exists() {
        std::fs::create_dir_all(&config_home_path).unwrap();
    }

    Ok(config_home_path)
}
