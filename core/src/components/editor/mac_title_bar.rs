use tiny_skia::{Color, FillRule, Paint, PathBuilder, Transform};

use crate::{
    components::interface::{
        component::{Component, ComponentContext, RenderParams},
        render_error,
        style::{ComponentStyle, RawComponentStyle, Size, Style},
    },
    edges::margin::Margin,
};

const RADIUS: f32 = 6.;

pub struct MacTitleBar {
    children: Vec<Box<dyn Component>>,
    render_condition: bool,
}

impl Component for MacTitleBar {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self, _context: &ComponentContext) -> RawComponentStyle {
        let demeter = RADIUS * 2.;

        Style::default()
            .size(Size::Num(demeter + 2. * 25.), Size::Num(demeter))
            .margin(Margin {
                bottom: 10.,
                ..Margin::default()
            })
    }

    fn render_condition(&self, _context: &ComponentContext) -> bool {
        return self.render_condition;
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        _style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        self.draw_control_buttons(
            // Control bar construct by draw circles, after drawn, the path will be at the center,
            // so the x, y need to offset by radius of the circle, and the next shape will still
            // be drwan on the original point
            render_params.x + RADIUS,
            render_params.y + RADIUS,
            pixmap,
            vec![
                Color::from_rgba8(255, 94, 87, 255),
                Color::from_rgba8(255, 186, 46, 255),
                Color::from_rgba8(43, 200, 65, 255),
            ],
            4.,
            Transform::from_scale(context.scale_factor, context.scale_factor),
        );

        Ok(())
    }

    fn name(&self) -> &'static str {
        "MacTitleBar"
    }
}

impl MacTitleBar {
    pub fn new(render_condition: bool) -> MacTitleBar {
        MacTitleBar {
            children: vec![],
            render_condition,
        }
    }

    fn draw_control_buttons(
        &self,
        x: f32,
        y: f32,
        pixmap: &mut tiny_skia::Pixmap,
        colors: Vec<Color>,
        gap: f32,
        transform: Transform,
    ) {
        for (index, color) in colors.into_iter().enumerate() {
            let diameter = RADIUS * 2.;

            self.draw_control_button(
                x,
                y,
                pixmap,
                color,
                index as f32 * (diameter + gap),
                transform,
            );
        }
    }

    fn draw_control_button(
        &self,
        x: f32,
        y: f32,
        pixmap: &mut tiny_skia::Pixmap,
        color: Color,
        x_offset: f32,
        transform: Transform,
    ) {
        let mut path_builder = PathBuilder::new();

        path_builder.push_circle(x + x_offset, y, RADIUS);
        path_builder.close();

        let path = path_builder.finish().unwrap();
        let mut paint = Paint::default();

        paint.set_color(color);
        pixmap.fill_path(&path, &paint, FillRule::Winding, transform, None);
    }
}
