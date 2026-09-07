use cosmic_text::{Attrs, Family, Style, Weight};
use syntect::{
    easy::HighlightLines, highlighting::FontStyle, parsing::SyntaxSet, util::LinesWithEndings,
};

use crate::components::interface::render_error::RenderError;

pub struct Highlight {
    content: String,
    font_family: String,
}

pub type HighlightResult<'a> = Vec<(&'a str, Attrs<'a>)>;

impl Highlight {
    pub fn new(content: String, font_family: String) -> Highlight {
        Highlight {
            content,
            font_family,
        }
    }

    // Parse Syntect Highlightlines to Cosmic Text span Attrs
    pub fn parse(
        &self,
        highlight: &mut HighlightLines,
        syntax_set: &SyntaxSet,
    ) -> Result<Vec<(&str, Attrs)>, RenderError> {
        let attrs = Attrs::new().family(Family::Name(self.font_family.as_ref()));

        let mut spans = Vec::new();
        for line in LinesWithEndings::from(&self.content) {
            for (style, text) in highlight.highlight_line(line, syntax_set).unwrap() {
                let syntect::highlighting::Color { r, g, b, a } = style.foreground;
                let attrs = match style.font_style {
                    FontStyle::BOLD => attrs.clone().weight(Weight::BOLD),
                    FontStyle::ITALIC => attrs.clone().style(Style::Italic),
                    _ => attrs.clone(),
                };
                spans.push((text, attrs.color(cosmic_text::Color::rgba(r, g, b, a))));
            }
        }

        Ok(spans)
    }
}
