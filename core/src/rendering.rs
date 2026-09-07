use std::sync::{Arc, Mutex};

use cosmic_text::{Align, Attrs, Metrics};
use tiny_skia::{
    FillRule, LinearGradient, Paint, Path, PathBuilder, Pixmap, PixmapPaint, Point, Rect,
    SpreadMode, Transform,
};

use crate::utils::text::{FontRenderer, PreparedText};
use crate::{
    config::Background,
    utils::{color::RgbaColor, helpers::convert_vecs},
};

pub mod shadow;
use shadow::Shadow;

enum Draw {
    BackgroundRow(Vec<u8>),
    Rect(Rect, Paint<'static>),
    Path(Path, Paint<'static>, Transform),
    Image(i32, i32, Arc<Pixmap>, PixmapPaint, Transform),
    Text(f32, f32, PreparedText),
    Shadow(Shadow),
}

pub struct Scene {
    width: u32,
    height: u32,
    draws: Vec<Draw>,
    font_renderer: Arc<Mutex<FontRenderer>>,
}

impl Scene {
    pub fn new(width: u32, height: u32, font_renderer: Arc<Mutex<FontRenderer>>) -> Self {
        Self {
            width,
            height,
            draws: Vec::new(),
            font_renderer,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn fill_rect(&mut self, rect: Rect, paint: Paint<'static>, transform: Transform) {
        if transform.is_identity() && self.width <= 8191 && self.height <= 8191 {
            let rect = if paint.anti_alias {
                rect
            } else {
                let Some(rect) = rect.round() else {
                    return;
                };

                rect.to_rect()
            };
            self.draws.push(Draw::Rect(rect, paint));
        } else {
            self.fill_path(PathBuilder::from_rect(rect), paint, transform);
        }
    }

    pub fn background(&mut self, background: &Background) {
        assert!(self.draws.is_empty());

        let mut paint = Paint {
            anti_alias: false,
            ..Paint::default()
        };
        let width = self.width as f32;
        let height = self.height as f32;
        let repeats = match background {
            Background::Solid(color) => {
                let color: RgbaColor = color.as_str().into();
                paint.set_color(color.into());

                true
            }
            Background::Gradient(gradient) => {
                // Resolve "max" against the whole canvas, never a PNG strip
                let start = gradient.start.into_f32_point(width, height);
                let end = gradient.end.into_f32_point(width, height);
                paint.shader = LinearGradient::new(
                    Point::from_xy(start.x, start.y),
                    Point::from_xy(end.x, end.y),
                    convert_vecs(gradient.stops.clone()),
                    SpreadMode::Pad,
                    Transform::identity(),
                )
                .unwrap();

                start.y == end.y
            }
        };
        if repeats {
            let mut row = Pixmap::new(self.width, 1).unwrap();
            row.fill_rect(
                Rect::from_xywh(0., 0., width, 1.).unwrap(),
                &paint,
                Transform::identity(),
                None,
            );
            self.draws.push(Draw::BackgroundRow(row.take()));
        } else {
            self.fill_rect(
                Rect::from_xywh(0., 0., width, height).unwrap(),
                paint,
                Transform::identity(),
            );
        }
    }

    pub fn fill_path(&mut self, path: Path, paint: Paint<'static>, transform: Transform) {
        self.draws.push(Draw::Path(path, paint, transform));
    }

    pub fn draw_image(
        &mut self,
        x: i32,
        y: i32,
        image: Arc<Pixmap>,
        paint: PixmapPaint,
        transform: Transform,
    ) {
        self.draws.push(Draw::Image(x, y, image, paint, transform));
    }

    pub fn draw_shadow(&mut self, shadow: Shadow) {
        self.draws.push(Draw::Shadow(shadow));
    }

    pub fn draw_text(&mut self, x: f32, y: f32, metrics: Metrics, spans: Vec<(&str, Attrs)>) {
        let text = self
            .font_renderer
            .lock()
            .unwrap()
            .prepare_text(metrics, spans);
        self.draws.push(Draw::Text(x, y, text));
    }

    pub fn draw_line(
        &mut self,
        x: f32,
        y: f32,
        metrics: Metrics,
        line: &str,
        attrs: Attrs,
        align: Option<Align>,
    ) {
        let text = self.font_renderer.lock().unwrap().prepare_line(
            metrics,
            line,
            attrs,
            align,
            self.width,
            self.height,
        );
        self.draws.push(Draw::Text(x, y, text));
    }

    pub fn render(&self) -> anyhow::Result<Pixmap> {
        self.render_strip(0, self.height)
    }

    pub fn render_strip(&self, y: u32, height: u32) -> anyhow::Result<Pixmap> {
        assert!(height > 0 && y <= self.height && height <= self.height - y);

        let mut pixmap = Pixmap::new(self.width, height).ok_or_else(|| {
            anyhow::anyhow!("Cannot allocate {}x{} snapshot strip", self.width, height)
        })?;
        for draw in &self.draws {
            match draw {
                Draw::BackgroundRow(row) => {
                    for destination in pixmap.data_mut().chunks_exact_mut(row.len()) {
                        destination.copy_from_slice(row);
                    }
                }
                Draw::Rect(rect, paint) => {
                    let rect = if paint.anti_alias {
                        *rect
                    } else {
                        // tiny-skia rounds negative coordinates differently;
                        // clip the already-rounded global rectangle first
                        let clip = Rect::from_xywh(0., y as f32, self.width as f32, height as f32)
                            .unwrap();
                        let Some(rect) = rect.intersect(&clip) else {
                            continue;
                        };

                        rect
                    };
                    let mut paint = paint.clone();
                    paint
                        .shader
                        .transform(Transform::from_translate(0., -(y as f32)));
                    let rect =
                        Rect::from_xywh(rect.x(), rect.y() - y as f32, rect.width(), rect.height())
                            .unwrap();
                    pixmap.fill_rect(rect, &paint, Transform::identity(), None);
                }
                Draw::Path(path, paint, transform) => {
                    pixmap.fill_path(
                        path,
                        paint,
                        FillRule::Winding,
                        transform.post_translate(0., -(y as f32)),
                        None,
                    );
                }
                Draw::Image(x, image_y, image, paint, transform) => {
                    pixmap.draw_pixmap(
                        *x,
                        *image_y,
                        image.as_ref().as_ref(),
                        paint,
                        transform.post_translate(0., -(y as f32)),
                        None,
                    );
                }
                Draw::Text(x, text_y, text) => {
                    self.font_renderer.lock().unwrap().paint_text(
                        *x,
                        *text_y,
                        text,
                        &mut pixmap,
                        y,
                        self.height,
                    );
                }
                Draw::Shadow(shadow) => shadow.paint(&mut pixmap, y, self.height)?,
            }
        }

        Ok(pixmap)
    }
}
