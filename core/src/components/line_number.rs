use super::{
    editor::code::CODE_LINE_HEIGHT,
    interface::{
        component::{Component, ComponentContext, RenderParams},
        render_error,
        style::{ComponentStyle, RawComponentStyle, Size, Style},
    },
};
use crate::{config::Code, edges::margin::Margin, utils::code::CHAR_WIDTH};
use cosmic_text::{Attrs, Color, Family, Metrics};

#[derive(Default)]
pub struct LineNumber {
    children: Vec<Box<dyn Component>>,
    render_condition: bool,
    line_number_content: Vec<String>,
    number_of_digit: usize,
}

impl Component for LineNumber {
    fn name(&self) -> &'static str {
        "LineNumber"
    }

    fn render_condition(&self, _context: &ComponentContext) -> bool {
        return self.render_condition;
    }

    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self, _context: &ComponentContext) -> RawComponentStyle {
        Style::default()
            .size(
                Size::Num(CHAR_WIDTH * self.number_of_digit as f32),
                Size::Num(self.line_number_content.len() as f32 * CODE_LINE_HEIGHT),
            )
            .margin(Margin {
                right: 10.,
                ..Margin::default()
            })
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        _style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        context.font_renderer.lock().unwrap().draw_text(
            render_params.x,
            render_params.y,
            Metrics::new(14., CODE_LINE_HEIGHT),
            vec![(
                &self.line_number_content.join("\n"),
                Attrs::new()
                    .color(Color::rgb(73, 81, 98))
                    .family(Family::Name(
                        &context.take_snapshot_params.code_config.font_family,
                    )),
            )],
            pixmap,
        );

        Ok(())
    }
}

impl LineNumber {
    pub fn new(code_content: Code) -> LineNumber {
        match code_content.start_line_number {
            None => LineNumber::default(),
            Some(ref start_line_number) => {
                let lines = code_content.content.split("\n").collect::<Vec<&str>>();
                let max_line_number = lines.len() as u32 + start_line_number;
                let number_of_digit = (max_line_number - 1).to_string().len();

                LineNumber {
                    line_number_content: (*start_line_number..max_line_number)
                        .map(|line_number| {
                            format!(
                                "{:>width$}",
                                line_number.to_string(),
                                width = number_of_digit
                            )
                        })
                        .collect::<Vec<String>>(),
                    number_of_digit,
                    children: vec![],
                    render_condition: true,
                }
            }
        }
    }
}
