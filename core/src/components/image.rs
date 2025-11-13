use tiny_skia::{Pixmap, PixmapPaint};

use crate::components::interface::{
    component::{Component, ComponentContext, RenderParams},
    render_error,
    style::{ComponentStyle, RawComponentStyle, Size, Style},
};

pub struct Image {
    image_pixmap: Pixmap,
    children: Vec<Box<dyn Component>>,
}

impl Component for Image {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self, _context: &ComponentContext) -> RawComponentStyle {
        RawComponentStyle::default().size(
            Size::Num(self.image_pixmap.width() as f32),
            Size::Num(self.image_pixmap.height() as f32),
        )
    }

    fn draw_self(
        &self,
        pixmap: &mut Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        _style: &ComponentStyle,
        _parent_style: &Style<f32>,
    ) -> render_error::Result<()> {
        let transform =
            tiny_skia::Transform::from_scale(context.scale_factor, context.scale_factor);
        let paint = PixmapPaint::default();

        pixmap.draw_pixmap(
            render_params.x as i32,
            render_params.y as i32,
            self.image_pixmap.as_ref(),
            &paint,
            transform,
            None,
        );

        Ok(())
    }
}

impl Image {
    pub fn new(image_data: Vec<u8>) -> anyhow::Result<Self> {
        let image_pixmap = Pixmap::decode_png(&image_data)?;

        Ok(Self {
            image_pixmap,
            children: vec![],
        })
    }
}
