use syntect::{
    dumps::from_binary,
    easy::HighlightLines,
    highlighting::{Color, Theme, ThemeSet},
    parsing::{SyntaxReference, SyntaxSet},
};

use crate::{
    components::interface::render_error::{self, RenderError},
    config::SnapshotConfig,
};

const CANDY_THEME: &[u8] = include_bytes!("../../assets/themes/candy.themedump");

pub struct ThemeColor(Color);

#[derive(Debug, Clone)]
pub struct ThemeProvider {
    pub theme: Theme,
    pub syntax: SyntaxReference,
    pub syntax_set: SyntaxSet,
}

impl Into<tiny_skia::Color> for ThemeColor {
    fn into(self) -> tiny_skia::Color {
        tiny_skia::Color::from_rgba8(self.0.r, self.0.g, self.0.b, self.0.a)
    }
}

impl ThemeProvider {
    fn guess_syntax(
        syntax_set: &SyntaxSet,
        language: Option<String>,
        code_file_path: Option<String>,
        code: &str,
    ) -> Result<SyntaxReference, RenderError> {
        let syntax = match &language {
            Some(language) => syntax_set.find_syntax_by_token(&language),
            None => match &code_file_path {
                Some(file_path) => syntax_set
                    .find_syntax_for_file(&file_path)
                    .map_err(|_| RenderError::NoSuchFile(file_path.clone()))?,
                None => syntax_set.find_syntax_by_first_line(code),
            },
        }
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

        Ok(syntax.to_owned())
    }

    pub fn from(
        themes_folder: Option<String>,
        theme: &str,
        language: Option<String>,
        code_file_path: Option<String>,
        code: &str,
    ) -> render_error::Result<ThemeProvider> {
        let theme_set = match themes_folder {
            Some(theme_folder) => ThemeSet::load_from_folder(theme_folder)
                .map_err(|_| RenderError::HighlightThemeLoadFailed)?,
            None => from_binary(CANDY_THEME),
        };
        let theme = theme_set.themes.get(theme).unwrap().to_owned();
        let syntax_set = two_face::syntax::extra_newlines();
        let syntax = ThemeProvider::guess_syntax(&syntax_set, language, code_file_path, code)?;

        Ok(ThemeProvider {
            theme,
            syntax,
            syntax_set,
        })
    }

    pub fn from_config(config: &SnapshotConfig) -> render_error::Result<ThemeProvider> {
        ThemeProvider::from(
            config.themes_folder.clone(),
            &config.code.theme,
            config.code.language.clone(),
            config.code.file_path.clone(),
            &config.code.content,
        )
    }

    pub fn highlight(&self) -> (HighlightLines, &SyntaxSet) {
        (
            HighlightLines::new(&self.syntax, &self.theme),
            &self.syntax_set,
        )
    }

    pub fn theme_background(&self) -> ThemeColor {
        ThemeColor(self.theme.settings.background.unwrap())
    }
}
