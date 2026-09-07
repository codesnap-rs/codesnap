use crate::utils::blur::{apply, support_radius, ImageRefMut};
use rgb::FromSlice;
use std::ops::Range;
use tiny_skia::{
    Color, FillRule, FilterQuality, IntRect, Paint, PathBuilder, Pattern, Pixmap, Rect, SpreadMode,
    Transform,
};

pub struct Shadow {
    bounds: IntRect,
    shape: Rect,
    color: Color,
    sigma: f64,
    halo: u32,
    scale: f32,
    uniform: Option<(Range<u32>, Vec<u8>)>,
}

impl Shadow {
    pub fn new(bounds: Rect, shape: Rect, color: Color, sigma: f32, scale: f32) -> Self {
        let width = bounds.width() as u32;
        let height = bounds.height() as u32;
        // Truncate source coordinates before scaling, as draw_pixmap does
        let bounds =
            IntRect::from_xywh(bounds.x() as i32, bounds.y() as i32, width, height).unwrap();
        let halo = support_radius(sigma as f64);
        let mut shadow = Self {
            bounds,
            shape,
            color,
            sigma: sigma as f64,
            halo,
            scale,
            uniform: None,
        };
        let solid_first = shape.y().ceil().max(0.0) as u32;
        let solid_end = (shape.bottom().floor().max(0.0) as u32).min(height);
        let uniform_first = solid_first.saturating_add(halo);
        let uniform_end = solid_end.saturating_sub(halo);
        if uniform_first < uniform_end {
            // Interior rows need only horizontal blur, preserving per-pass rounding
            // Rasterize neighboring rows too: a one-row clip changes rectangle AA
            let first = uniform_first.saturating_sub(1);
            let rows = uniform_first.saturating_add(2).min(height) - first;
            let mut row = Pixmap::new(width, rows).expect("invalid shadow dimensions");
            shadow.fill_source(&mut row, first);
            apply(
                sigma as f64,
                0.0,
                ImageRefMut::new(width, rows, row.data_mut().as_rgba_mut()),
            );
            let row_size = width as usize * 4;
            let offset = (uniform_first - first) as usize * row_size;
            shadow.uniform = Some((
                uniform_first..uniform_end,
                row.data()[offset..offset + row_size].to_vec(),
            ));
        }

        shadow
    }

    fn fill_source(&self, source: &mut Pixmap, first: u32) {
        let mut paint = Paint::default();
        paint.set_color(self.color);
        let transform = Transform::from_translate(0.0, -(first as f32));
        if self.bounds.width() > 8191 || self.bounds.height() > 8191 {
            // Match the full canvas's rasterizer above tiny-skia's 8191px threshold
            source.fill_path(
                &PathBuilder::from_rect(self.shape),
                &paint,
                FillRule::Winding,
                transform,
                None,
            );
        } else {
            // Translate geometry to retain rectangle AA; a transform selects fill_path
            source.fill_rect(
                self.shape.transform(transform).unwrap(),
                &paint,
                Transform::identity(),
                None,
            );
        }
    }

    pub fn paint(
        &self,
        target: &mut Pixmap,
        global_y: u32,
        canvas_height: u32,
    ) -> anyhow::Result<()> {
        let first = (global_y as f32 / self.scale - self.bounds.y() as f32)
            .floor()
            .max(0.0) as u32;
        // Keep the last source row for Pattern::Pad at rounded mask edges
        let first = first.min(self.bounds.height().saturating_sub(1));
        let end = ((global_y + target.height()) as f32 / self.scale - self.bounds.y() as f32)
            .ceil()
            .max(0.0) as u32;
        let end = end.min(self.bounds.height());
        if first >= end {
            return Ok(());
        }

        let uniform = self
            .uniform
            .as_ref()
            .filter(|(rows, _)| first >= rows.start && end <= rows.end);
        // Extend strips by the blur support plus one row for rectangle AA
        // Clamp to the source canvas to preserve zero padding at its real edges
        let halo = self.halo.saturating_add(1);
        let (first, end) = if uniform.is_some() {
            (first, end)
        } else {
            (
                first.saturating_sub(halo),
                end.saturating_add(halo).min(self.bounds.height()),
            )
        };
        let mut source = Pixmap::new(self.bounds.width(), end - first)
            .ok_or_else(|| anyhow::anyhow!("failed to allocate shadow strip"))?;
        if let Some((_, pixels)) = uniform {
            for row in source
                .data_mut()
                .chunks_exact_mut(self.bounds.width() as usize * 4)
            {
                row.copy_from_slice(pixels);
            }
        } else {
            self.fill_source(&mut source, first);
            apply(
                self.sigma,
                self.sigma,
                ImageRefMut::new(
                    self.bounds.width(),
                    end - first,
                    source.data_mut().as_rgba_mut(),
                ),
            );
        }

        // Keep full source bounds: cropping changes tiny-skia's coordinate rounding
        // and rectangle/path rasterizer selection
        let bounds = self.bounds.to_rect();
        let mut paint = Paint {
            shader: Pattern::new(
                source.as_ref(),
                SpreadMode::Pad,
                FilterQuality::Nearest,
                1.0,
                Transform::from_translate(
                    self.bounds.x() as f32,
                    (self.bounds.y() + first as i32) as f32,
                ),
            ),
            anti_alias: false,
            ..Paint::default()
        };
        if self.scale == 1.0 && target.width() <= 8191 && canvas_height <= 8191 {
            let clip =
                IntRect::from_xywh(0, global_y as i32, target.width(), target.height()).unwrap();
            if let Some(bounds) = bounds.round().unwrap().intersect(&clip) {
                paint
                    .shader
                    .transform(Transform::from_translate(0.0, -(global_y as f32)));
                target.fill_rect(
                    bounds.translate(0, -(global_y as i32)).unwrap().to_rect(),
                    &paint,
                    Transform::identity(),
                    None,
                );
            }
        } else {
            target.fill_path(
                &PathBuilder::from_rect(bounds),
                &paint,
                FillRule::Winding,
                Transform::from_scale(self.scale, self.scale)
                    .post_translate(0.0, -(global_y as f32)),
                None,
            );
        }

        Ok(())
    }
}
