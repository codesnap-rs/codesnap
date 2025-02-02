use cansi::v3::{categorise_text, Color};
use cosmic_text::{Attrs, Family};

use crate::utils::color::parse_hex_to_cosmic_color;

pub struct ANSI {
    raw_text: String,
    font_family: String,
}

impl ANSI {
    pub fn from(text: &str, font_family: String) -> Self {
        Self {
            raw_text: text.to_string(),
            font_family,
        }
    }

    // Parse Cansi categories to Cosmic Text span Attrs
    pub fn colorize(&self) -> Vec<(&str, Attrs)> {
        categorise_text(&self.raw_text)
            .into_iter()
            .map(|category| {
                let hex_color = parse_color_to_hex(category.fg.unwrap_or(Color::White));

                (
                    category.text,
                    Attrs::new()
                        .color(parse_hex_to_cosmic_color(hex_color))
                        .family(Family::Name(self.font_family.as_str())),
                )
            })
            .collect::<Vec<(&str, Attrs)>>()
    }
}

// Parse Cansi color to string
pub fn parse_color_to_hex(color: Color) -> &'static str {
    match color {
        Color::Black => "#4B4B4B",
        Color::Red => "#FF6F61",
        Color::Green => "#77DD77",
        Color::Yellow => "#FFEB3B",
        Color::Blue => "#89CFF0",
        Color::Magenta => "#FF77FF",
        Color::Cyan => "#00FFFF",
        Color::White => "#979EAB",
        Color::BrightBlack => "#696969",
        Color::BrightRed => "#FF9999",
        Color::BrightGreen => "#99FF99",
        Color::BrightYellow => "#FFFF99",
        Color::BrightBlue => "#ADD8E6",
        Color::BrightMagenta => "#FFB6C1",
        Color::BrightCyan => "#E0FFFF",
        Color::BrightWhite => "#F5F5F5",
    }
}
