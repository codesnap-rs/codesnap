use anyhow::anyhow;
use tiny_skia::{FillRule, FilterQuality, Mask, Path, PathBuilder, Pixmap, PixmapPaint, Transform};

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
        let transform = Transform::from_scale(context.scale_factor, context.scale_factor);
        let mut paint = PixmapPaint::default();

        paint.quality = FilterQuality::Bilinear;

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
    pub fn new(radius: f32, image_data: Vec<u8>) -> anyhow::Result<Self> {
        let mut image_pixmap = Pixmap::decode_png(&image_data)?;
        let rounded_mask = Self::build_round_mask(&image_pixmap, radius)?;
        image_pixmap.apply_mask(&rounded_mask);

        Ok(Self {
            image_pixmap,
            children: vec![],
        })
    }

    fn build_round_mask(pixmap: &Pixmap, radius: f32) -> anyhow::Result<Mask> {
        let width = pixmap.width();
        let height = pixmap.height();

        if width == 0 || height == 0 {
            return Err(anyhow!("image pixmap must have non-zero dimensions"));
        }

        let path = Self::rounded_rect_path(width as f32, height as f32, radius)?;
        let mut mask = Mask::new(width, height).ok_or_else(|| anyhow!("failed to create mask"))?;
        mask.fill_path(&path, FillRule::Winding, true, Transform::identity());

        Ok(mask)
    }

    fn rounded_rect_path(width: f32, height: f32, radius: f32) -> anyhow::Result<Path> {
        if width <= 0.0 || height <= 0.0 {
            return Err(anyhow!("rounded rect requires positive size"));
        }

        let mut builder = PathBuilder::new();
        let radius = radius.max(0.0).min(width / 2.0).min(height / 2.0);
        let right = width;
        let bottom = height;

        builder.move_to(radius, 0.0);
        builder.line_to(right - radius, 0.0);
        builder.quad_to(right, 0.0, right, radius);
        builder.line_to(right, bottom - radius);
        builder.quad_to(right, bottom, right - radius, bottom);
        builder.line_to(radius, bottom);
        builder.quad_to(0.0, bottom, 0.0, bottom - radius);
        builder.line_to(0.0, radius);
        builder.quad_to(0.0, 0.0, radius, 0.0);
        builder.close();

        builder
            .finish()
            .ok_or_else(|| anyhow!("failed to create rounded rectangle path"))
    }
}
