use include_dir::{include_dir, Dir};

static THEME_CONFIGS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/theme_configs");

pub fn get_theme(theme_name: &str) -> String {
    THEME_CONFIGS
        .get_file(format!("{}.json", theme_name))
        .unwrap()
        .contents_utf8()
        .unwrap()
        .to_string()
}
