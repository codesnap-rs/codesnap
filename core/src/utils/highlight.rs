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
                        let syntect::highlighting::Color { r, g, b, a: _ } = style.foreground;
                        let attrs = match style.font_style {
                            FontStyle::BOLD => attrs.weight(Weight::BOLD),
                            FontStyle::ITALIC => attrs.style(Style::Italic),
                            FontStyle::UNDERLINE => attrs.style(Style::Normal),
                            _ => attrs,
                        };

                        (str, attrs.color(cosmic_text::Color::rgb(r, g, b)))
                    })
                    .collect::<HighlightResult>()
            })
            .fold(vec![], |acc, cur| [acc, cur].concat())
            .into_iter()
            .collect::<HighlightResult>())
    }
}
