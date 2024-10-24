use syntect::highlighting::ThemeSet;

use crate::components::interface::render_error::{RenderError, Result};

pub fn get_theme_set_from_folder(theme_folder: Option<String>) -> Result<ThemeSet> {
    let theme_set = match theme_folder {
        Some(theme_folder) => ThemeSet::load_from_folder(theme_folder)
            .map_err(|_| RenderError::HighlightThemeLoadFailed)?,
        None => ThemeSet::load_defaults(),
    };

    Ok(theme_set)
}
