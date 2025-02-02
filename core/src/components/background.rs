use tiny_skia::{LinearGradient, Paint, Pixmap, Point, Rect, SpreadMode, Transform};

use crate::{
    edges::{edge::Edge, padding::Padding},
    utils::{color::RgbaColor, helpers::convert_vecs},
};

use super::interface::{
    component::{Component, ComponentContext, RenderParams},
    render_error::{self},
    style::{ComponentAlign, ComponentStyle, RawComponentStyle},
};

pub struct Background {
    children: Vec<Box<dyn Component>>,
    padding: Padding,
}

impl Background {
    pub fn new(padding: Padding, children: Vec<Box<dyn Component>>) -> Background {
        Background { children, padding }
    }

    pub fn has_background(padding: &Padding) -> bool {
        return padding.horizontal() != 0. || padding.vertical() != 0.;
    }
}

impl Component for Background {
    fn name(&self) -> &'static str {
        "Background"
    }

    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self, _context: &ComponentContext) -> RawComponentStyle {
        RawComponentStyle::default()
            .align(ComponentAlign::Column)
            .padding(self.padding.clone())
    }

    fn self_render_condition(&self, _context: &ComponentContext) -> bool {
        Self::has_background(&self.padding)
    }

    fn draw_self(
        &self,
        pixmap: &mut Pixmap,
        context: &ComponentContext,
        _render_params: &RenderParams,
        _style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        let mut paint = Paint::default();
        let w = pixmap.width() as f32;
        let h = pixmap.height() as f32;
        let params = &context.take_snapshot_params;

        paint.anti_alias = false;

        match &params.background {
            crate::config::Background::Solid(solid_background) => {
                let rgba_color: RgbaColor = solid_background.as_str().into();

                paint.set_color(rgba_color.into());
            }
            crate::config::Background::Gradient(gradient_background) => {
                let start = gradient_background.start.into_f32_point(w, h);
                let end = gradient_background.end.into_f32_point(w, h);

                paint.shader = LinearGradient::new(
                    Point::from_xy(start.x, start.y),
                    Point::from_xy(end.x, end.y),
                    convert_vecs(gradient_background.stops.clone()),
                    SpreadMode::Pad,
                    Transform::identity(),
                )
                .unwrap();
            }
        };

        pixmap.fill_rect(
            Rect::from_xywh(0., 0., w, h).unwrap(),
            &paint,
            Transform::identity(),
            None,
        );

        Ok(())
    }
}
