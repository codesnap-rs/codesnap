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

        // Highlight the content line by line using highlight_line function
        Ok(LinesWithEndings::from(&self.content)
            .map(|line| {
                highlight
                    .highlight_line(line, &syntax_set)
                    .unwrap()
                    .into_iter()
                    .map(|(style, str)| {
                        let syntect::highlighting::Color { r, g, b, a } = style.foreground;
                        let attrs_cloned = attrs.clone();
                        let attrs = match style.font_style {
                            FontStyle::BOLD => attrs_cloned.weight(Weight::BOLD),
                            FontStyle::ITALIC => attrs_cloned.style(Style::Italic),
                            FontStyle::UNDERLINE => attrs_cloned.style(Style::Normal),
                            _ => attrs_cloned,
                        };

                        (str, attrs.color(cosmic_text::Color::rgba(r, g, b, a)))
                    })
                    .collect::<HighlightResult>()
            })
            .fold(vec![], |acc, cur| [acc, cur].concat())
            .into_iter()
            .collect::<HighlightResult>())
    }
}
