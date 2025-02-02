use crate::{config::HighlightLine, edges::padding::Padding, utils::color::RgbaColor};

use super::{
    editor::code::CODE_LINE_HEIGHT,
    interface::{
        component::{Component, ComponentContext, RenderParams},
        style::ComponentStyle,
    },
};
use tiny_skia::{Paint, Rect, Transform};

#[derive(Default)]
pub struct HighlightCodeBlock {
    children: Vec<Box<dyn Component>>,
    highlight_lines: Vec<HighlightLine>,
    editor_padding: Padding,
    code_line_count: usize,
}

impl Component for HighlightCodeBlock {
    fn name(&self) -> &'static str {
        "HighlightCodeBlock"
    }

    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn render_condition(&self, _context: &ComponentContext) -> bool {
        self.highlight_lines.len() > 0
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &super::interface::component::ComponentContext,
        render_params: &super::interface::component::RenderParams,
        _style: &super::interface::style::ComponentStyle,
        parent_style: &ComponentStyle,
    ) -> super::interface::render_error::Result<()> {
        for highlight_line in &self.highlight_lines {
            let (start_line_number, end_line_number, color) = match highlight_line {
                HighlightLine::Single(line_number, color) => (line_number, line_number, color),
                HighlightLine::Range(start_line_number, end_line_number, color) => {
                    (start_line_number, end_line_number, color)
                }
            };
            let (rect, paint) = self.draw_highlight_line(
                render_params,
                parent_style,
                *start_line_number,
                *end_line_number,
                color,
            );

            pixmap.fill_rect(
                rect,
                &paint,
                Transform::from_scale(context.scale_factor, context.scale_factor),
                None,
            );
        }

        Ok(())
    }
}

impl HighlightCodeBlock {
    pub fn from(
        highlight_lines: Vec<HighlightLine>,
        code_line_count: usize,
        editor_padding: Padding,
    ) -> HighlightCodeBlock {
        HighlightCodeBlock {
            children: vec![],
            code_line_count,
            highlight_lines,
            editor_padding,
        }
    }

    fn draw_highlight_line(
        &self,
        render_params: &RenderParams,
        parent_style: &ComponentStyle,
        start_line_number: u32,
        end_line_number: u32,
        hex: &str,
    ) -> (Rect, Paint) {
        // If the start_line_number is greater than end_line_number, swap them
        if start_line_number > end_line_number {
            return self.draw_highlight_line(
                render_params,
                parent_style,
                end_line_number,
                start_line_number,
                hex,
            );
        }

        let end_line_number = end_line_number.min(self.code_line_count as u32);
        let mut paint = Paint::default();
        // If the start line number is start at n, the y offset should be (n - 1) * line_height
        let start_y_offset = (start_line_number - 1) as f32 * CODE_LINE_HEIGHT;
        let rect = Rect::from_xywh(
            render_params.x - self.editor_padding.left,
            render_params.y + start_y_offset,
            parent_style.width,
            // If end_line_number is equal to start_line_number, the height should be line_height
            (end_line_number - start_line_number + 1) as f32 * CODE_LINE_HEIGHT,
        )
        .unwrap();
        let color: RgbaColor = hex.into();

        paint.set_color(color.into());
        paint.anti_alias = false;

        (rect, paint)
    }
}
