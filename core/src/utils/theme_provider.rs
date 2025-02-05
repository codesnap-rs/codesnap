use syntect::{
    dumps::from_binary,
    highlighting::{Color, Theme, ThemeSet},
};

use anyhow::Context;

use crate::{components::interface::render_error::RenderError, config::SnapshotConfig};

const PRESET_THEMES: &[u8] = include_bytes!("../../assets/code_themes/default.themedump");

pub struct ThemeColor(Color);

#[derive(Debug, Clone)]
pub struct ThemeProvider {
    pub theme: Theme,
}

impl Into<tiny_skia::Color> for ThemeColor {
    fn into(self) -> tiny_skia::Color {
        tiny_skia::Color::from_rgba8(self.0.r, self.0.g, self.0.b, self.0.a)
    }
}

impl ThemeProvider {
    pub fn from(themes_folder: Option<String>, theme: &str) -> anyhow::Result<ThemeProvider> {
        let mut theme_set: ThemeSet = from_binary(PRESET_THEMES);

        if let Some(theme_folder) = themes_folder {
            theme_set
                .add_from_folder(theme_folder)
                .map_err(|_| RenderError::HighlightThemeLoadFailed)?;
        }

        let theme = theme_set
            .themes
            .get(theme)
            .context(format!("Cannot find {} theme", theme))?
            .to_owned();

        Ok(ThemeProvider { theme })
    }

    pub fn from_config(config: &SnapshotConfig) -> anyhow::Result<ThemeProvider> {
        ThemeProvider::from(config.themes_folder.clone(), &config.theme)
    }

    pub fn theme_background(&self) -> ThemeColor {
        ThemeColor(self.theme.settings.background.unwrap())
    }
}
