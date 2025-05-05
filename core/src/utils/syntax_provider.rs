use hyperpolyglot::detectors::classify;
use syntect::parsing::{SyntaxReference, SyntaxSet};

use crate::components::interface::render_error::RenderError;

pub struct SyntaxProvider {
    pub syntax_set: SyntaxSet,
}

impl SyntaxProvider {
    pub fn guess_syntax(
        &self,
        language: Option<String>,
        code_file_path: Option<String>,
        code: &str,
    ) -> Result<SyntaxReference, RenderError> {
        let syntax = match &language {
            Some(language) => self.syntax_set.find_syntax_by_token(&language),
            None => match &code_file_path {
                Some(file_path) => self
                    .syntax_set
                    .find_syntax_for_file(&file_path)
                    .map_err(|_| RenderError::NoSuchFile(file_path.clone()))?,
                None => self.syntax_set.find_syntax_by_first_line(code),
            },
        }
        .or_else(|| {
            self.syntax_set
                .find_syntax_by_token(classify(code, &*vec![]))
        })
        .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        Ok(syntax.to_owned())
    }

    pub fn new() -> SyntaxProvider {
        let syntax_set = two_face::syntax::extra_newlines();

        SyntaxProvider { syntax_set }
    }
}
