use include_dir::{include_dir, Dir};
use std::fs;
use std::path::Path;
use theme_converter::{parser::Parser as ThemeParser, vscode};

use crate::{
    assets::{Assets, AssetsURL},
    utils::path::get_config_home_path,
};

static THEME_CONFIGS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/theme_configs");

pub fn get_theme(theme_name: &str) -> String {
    THEME_CONFIGS
        .get_file(format!("{}.json", theme_name))
        .unwrap()
        .contents_utf8()
        .unwrap()
        .to_string()
}

// If the code theme is URL, download it and return the path
// The code_theme can be a assets URL or a local file path
pub async fn parse_remote_code_theme(path: &str, code_theme: &str) -> anyhow::Result<String> {
    let assets_url = AssetsURL::from_url(code_theme);

    match assets_url {
        Ok(assets_url) => {
            let assets = Assets::from(path);
            let assets_store_path_str = assets.download(code_theme).await?;
            let assets_store_path = Path::new(&assets_store_path_str);
            let extension = assets_store_path.extension().unwrap_or_default();

            // If the code theme is JSON file, we treat it as a VSCode theme file
            if extension == "json" {
                let root = vscode::VSCodeThemeParser::from_config(&assets_store_path_str)
                    .unwrap()
                    .parse(&assets_url.name);
                let path = Path::new(&assets_store_path)
                    .with_file_name(format!("{}.{}", &assets_url.name, "tmTheme"));

                plist::to_writer_xml(&mut fs::File::create(path).unwrap(), &root)?;
            }

            Ok(assets_url.name)
        }
        Err(_) => {
            // If the code theme is not a URL, we will use it as a local file
            Ok(code_theme.to_string())
        }
    }
}

pub async fn parse_code_theme(code_theme: &str) -> anyhow::Result<String> {
    let remote_theme_folder_path = get_config_home_path().join("remote_themes");

    if !remote_theme_folder_path.exists() {
        fs::create_dir_all(&remote_theme_folder_path)?;
    }

    let remote_theme_folder_path_str = remote_theme_folder_path.to_string_lossy().to_string();

    parse_remote_code_theme(&remote_theme_folder_path_str, code_theme).await
}
