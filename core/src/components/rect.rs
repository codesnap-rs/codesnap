use super::interface::{
    component::{Component, ComponentContext, RenderParams},
    render_error,
    style::{ComponentAlign, ComponentStyle, RawComponentStyle, Style},
};
use crate::{
    edges::{edge::Edge, padding::Padding},
    utils::blur::{apply, ImageRefMut},
};
use rgb::FromSlice;
use tiny_skia::{
    Color, FillRule, Paint, PathBuilder, Pixmap, PixmapPaint, Rect as SkiaRect, Transform,
};

#[derive(Debug)]
struct ShadowConfig {
    x: f32,
    y: f32,
    blur: f32,
    color: Color,
}

pub struct Rect {
    radius: f32,
    min_width: f32,
    padding: Padding,
    color: Color,
    children: Vec<Box<dyn Component>>,
    shadow: Option<ShadowConfig>,
    component_name: &'static str,
}

impl Component for Rect {
    fn name(&self) -> &'static str {
        self.component_name
    }

    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self, _context: &ComponentContext) -> RawComponentStyle {
        Style::default()
            .min_width(self.min_width)
            .align(ComponentAlign::Column)
            .padding(self.padding.clone())
    }

    fn draw_self(
        &self,
        pixmap: &mut Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        let mut path_builder = PathBuilder::new();
        let x = render_params.x;
        let y = render_params.y;
        let w = style.width;
        let h = style.height;
        let rect_width = w - 2. * self.radius;
        let rect_height = h - 2. * self.radius;

        path_builder.move_to(x + self.radius, y);
        path_builder.line_to(x + self.radius + rect_width, y);
        path_builder.line_to(x + self.radius + rect_width, y + self.radius);

        path_builder.line_to(x + rect_width + self.radius * 2., y + self.radius);

        path_builder.line_to(
            x + rect_width + self.radius * 2.,
            y + rect_height + self.radius,
        );
        path_builder.line_to(x + rect_width + self.radius, y + rect_height + self.radius);
        path_builder.line_to(
            x + rect_width + self.radius,
            y + rect_height + self.radius * 2.,
        );

        path_builder.line_to(x + self.radius, y + rect_height + self.radius * 2.);
        path_builder.line_to(x + self.radius, y + rect_height + self.radius);

        path_builder.line_to(x, y + rect_height + self.radius);

        path_builder.line_to(x, y + self.radius);
        path_builder.line_to(x + self.radius, y + self.radius);
        path_builder.line_to(x + self.radius, y);
        path_builder.line_to(x + self.radius + rect_width, y);
        path_builder.push_circle(
            x + rect_width + self.radius,
            y + rect_height + self.radius,
            self.radius,
        );
        path_builder.push_circle(x + self.radius + rect_width, y + self.radius, self.radius);
        path_builder.push_circle(x + self.radius, y + self.radius, self.radius);
        path_builder.push_circle(x + self.radius, y + rect_height + self.radius, self.radius);
        path_builder.close();

        let path = path_builder.finish().unwrap();
        let mut paint = Paint::default();
        let transform = Transform::from_scale(context.scale_factor, context.scale_factor);

        paint.set_color(self.color);

        // Draw shadow element
        if let Some(shadow) = &self.shadow {
            self.draw_shadow(pixmap, render_params, shadow, transform, style, context);
        }

        pixmap.fill_path(&path, &paint, FillRule::Winding, transform, None);

        Ok(())
    }
}

impl Rect {
    pub fn new(
        radius: f32,
        color: Color,
        min_width: Option<f32>,
        padding: Padding,
        component_name: &'static str,
        children: Vec<Box<dyn Component>>,
    ) -> Rect {
        Rect {
            radius,
            color,
            children,
            padding,
            min_width: min_width.unwrap_or(0.),
            component_name,
            shadow: None,
        }
    }

    fn draw_shadow(
        &self,
        parent_pixmap: &mut Pixmap,
        render_params: &RenderParams,
        shadow: &ShadowConfig,
        transform: Transform,
        host_element_style: &ComponentStyle,
        context: &ComponentContext,
    ) {
        let w = host_element_style.width;
        let h = host_element_style.height;
        let config = context.take_snapshot_params.clone();
        let background_padding: Padding = config.window.margin.clone().into();
        // The shadow has a fixed length when blur is applied
        // thus the (shadow length) + (pixmap length) will out of the original pixmap
        // so we need to set a bigger pixmap to draw the shadow
        // the shadow_length is something like a "buffer" for drawing shadow
        // the original pixmap will draw at x and y, if we scale the pixmap with shadow_length
        // offset = (pixmap width - original pixmap width) / 2
        // x = x - offset
        // y = y - offset
        //
        // And we must draw the shadow object at center of the pixmap
        // so the shadow object should draw at:
        //
        // x = x + offset
        // y = y + offset
        let pixmap_w = w + background_padding.horizontal();
        let pixmap_h = h + background_padding.vertical();
        let pixmap_offset_x = (pixmap_w - w) / 2.;
        let pixmap_offset_y = (pixmap_h - h) / 2.;

        let mut pixmap = Pixmap::new(pixmap_w as u32, pixmap_h as u32).unwrap();
        let mut paint = Paint::default();
        let shadow_pixmap_x = render_params.x - pixmap_offset_x;
        let shadow_pixmap_y = render_params.y - pixmap_offset_y;

        paint.set_color(shadow.color);
        pixmap.fill_rect(
            SkiaRect::from_xywh(shadow.x + pixmap_offset_x, shadow.y + pixmap_offset_y, w, h)
                .unwrap(),
            &paint,
            Transform::identity(),
            None,
        );

        let rgba = pixmap.data_mut().as_rgba_mut();

        apply(
            shadow.blur as f64,
            shadow.blur as f64,
            ImageRefMut::new(pixmap_w as u32, pixmap_h as u32, rgba),
        );

        parent_pixmap.draw_pixmap(
            shadow_pixmap_x as i32,
            shadow_pixmap_y as i32,
            pixmap.as_ref(),
            &PixmapPaint::default(),
            transform,
            None,
        );
    }

    // The implementation of boder in CodeSnap is create a new Rect component with border color
    // And put it on the top of the current Rect component, the View like this:
    //
    // +------------------------------------+
    // |                                    | <- The color of this layer is same as inner rect
    // | +--------------------------------+ |
    // | |     This layer is border       | | (with semi-transparent color)
    // | | +----------------------------+ | |
    // | | |                            | | |
    // | | |                            | | |
    pub fn create_with_border(
        radius: f32,
        color: Color,
        min_width: f32,
        padding: Padding,
        border_width: f32,
        border_color: Color,
        children: Vec<Box<dyn Component>>,
    ) -> Rect {
        Rect::new(
            radius,
            color,
            Some(min_width + border_width * 2.),
            Padding::from_value(border_width),
            "RectUnderLayer",
            vec![Box::new(Rect::new(
                radius - border_width,
                border_color,
                Some(min_width + border_width),
                Padding::from_value(border_width),
                "RectBorderLayer",
                vec![Box::new(Rect::new(
                    radius - border_width * 2.,
                    color,
                    Some(min_width),
                    padding,
                    "RectInnerLayer",
                    children,
                ))],
            ))],
        )
    }

    pub fn shadow(mut self, x: f32, y: f32, blur: f32, color: Color) -> Rect {
        self.shadow = Some(ShadowConfig { x, y, blur, color });
        self
    }
}
