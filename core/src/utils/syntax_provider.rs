#[cfg(feature = "auto-detect")]
use hyperpolyglot_fork::detectors::classify;
use std::path::Path;
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
            None => {
                let by_path = code_file_path.as_deref().and_then(|p| {
                    let path = Path::new(p);
                    path.file_name()
                        .and_then(|n| n.to_str())
                        .and_then(|n| self.syntax_set.find_syntax_by_extension(n))
                        .or_else(|| {
                            path.extension()
                                .and_then(|e| e.to_str())
                                .and_then(|e| self.syntax_set.find_syntax_by_extension(e))
                        })
                });
                by_path.or_else(|| self.syntax_set.find_syntax_by_first_line(code))
            }
        };

        #[cfg(feature = "auto-detect")]
        let syntax = syntax.or_else(|| {
            self.syntax_set
                .find_syntax_by_token(classify(code, &*vec![]))
        });

        let syntax = syntax.unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        Ok(syntax.to_owned())
    }

    pub fn new() -> SyntaxProvider {
        let syntax_set = two_face::syntax::extra_newlines();

        SyntaxProvider { syntax_set }
    }
}
