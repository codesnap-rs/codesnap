use std::sync::Arc;

use cosmic_text::{
    fontdb::Source, Align, Attrs, AttrsList, Buffer, BufferLine, CacheKey, Color, Family,
    FontSystem, LineEnding, Metrics, Shaping, SwashCache,
};
use tiny_skia::{ColorU8, Pixmap};

const CASKAYDIA_COVE_NERD_FONT: &[u8] =
    include_bytes!("../../assets/fonts/CaskaydiaCoveNerdFont-Regular.ttf");
const CASKAYDIA_COVE_BOLD_NERD_FONT: &[u8] =
    include_bytes!("../../assets/fonts/CaskaydiaCoveNerdFont-Bold.ttf");
const CASKAYDIA_COVE_ITALIC_NERD_FONT: &[u8] =
    include_bytes!("../../assets/fonts/CaskaydiaCoveNerdFont-Italic.ttf");

const PACIFICO_FONT: &[u8] = include_bytes!("../../assets/fonts/Pacifico-Regular.ttf");

pub struct FontRenderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
    scale_factor: f32,
}

pub struct PreparedText {
    lines: Vec<PreparedLine>,
}

struct PreparedLine {
    top: i32,
    bottom: i32,
    glyphs: Vec<PreparedGlyph>,
}

struct PreparedGlyph {
    key: CacheKey,
    x: i32,
    y: i32,
    color: Color,
}

fn get_default_attrs<'a>() -> Attrs<'a> {
    Attrs::new().family(Family::Name("CaskaydiaCove Nerd Font"))
}

impl FontRenderer {
    pub fn new(scale_factor: f32, fonts_folders: Vec<String>) -> FontRenderer {
        let fonts_data = [
            CASKAYDIA_COVE_NERD_FONT,
            PACIFICO_FONT,
            CASKAYDIA_COVE_BOLD_NERD_FONT,
            CASKAYDIA_COVE_ITALIC_NERD_FONT,
        ];
        let sources = fonts_data
            .into_iter()
            .map(|data| Source::Binary(Arc::new(data)));

        let mut font_system = FontSystem::new_with_fonts(sources);
        let font_db = font_system.db_mut();

        for folder in fonts_folders {
            font_db.load_fonts_dir(folder);
        }

        FontRenderer {
            font_system,
            swash_cache: SwashCache::new(),
            scale_factor,
        }
    }

    pub fn measure_text(&mut self, metrics: Metrics, text: &str) -> (f32, f32) {
        let mut buffer = Buffer::new(&mut self.font_system, metrics.scale(self.scale_factor));

        buffer.set_text(
            &mut self.font_system,
            text,
            &get_default_attrs(),
            Shaping::Advanced,
        );

        let line_height = buffer.lines.len() as f32 * buffer.metrics().line_height;
        let width = buffer
            .layout_runs()
            .fold(0f32, |max_width, run| max_width.max(run.line_w));

        (
            width.ceil() / self.scale_factor,
            line_height / self.scale_factor,
        )
    }

    pub fn draw_text(
        &mut self,
        x: f32,
        y: f32,
        metrics: Metrics,
        spans: Vec<(&str, Attrs)>,
        pixmap: &mut Pixmap,
    ) {
        let text = self.prepare_text(metrics, spans);
        self.paint_text(x, y, &text, pixmap, 0, pixmap.height());
    }

    pub fn prepare_text(&mut self, metrics: Metrics, spans: Vec<(&str, Attrs)>) -> PreparedText {
        let mut buffer = Buffer::new(&mut self.font_system, metrics.scale(self.scale_factor));

        buffer.set_rich_text(
            &mut self.font_system,
            spans,
            &get_default_attrs(),
            Shaping::Advanced,
            None,
        );

        self.prepare_buffer(&buffer)
    }

    pub fn draw_line(
        &mut self,
        x: f32,
        y: f32,
        metrics: Metrics,
        line: &str,
        attrs: Attrs,
        align: Option<Align>,
        pixmap: &mut Pixmap,
    ) {
        let text = self.prepare_line(metrics, line, attrs, align, pixmap.width(), pixmap.height());
        self.paint_text(x, y, &text, pixmap, 0, pixmap.height());
    }

    pub fn prepare_line(
        &mut self,
        metrics: Metrics,
        line: &str,
        attrs: Attrs,
        align: Option<Align>,
        canvas_width: u32,
        canvas_height: u32,
    ) -> PreparedText {
        let mut buffer = Buffer::new(&mut self.font_system, metrics.scale(self.scale_factor));
        let attrs_list = AttrsList::new(&attrs);
        let ending = if cfg!(windows) {
            LineEnding::CrLf
        } else {
            LineEnding::Lf
        };
        let mut line = BufferLine::new(line, ending, attrs_list, Shaping::Advanced);

        line.set_align(align);
        buffer.lines = vec![line];
        buffer.set_size(
            &mut self.font_system,
            Some(canvas_width as f32),
            Some(canvas_height as f32),
        );

        self.prepare_buffer(&buffer)
    }

    fn prepare_buffer(&mut self, buffer: &Buffer) -> PreparedText {
        let mut lines = Vec::new();
        for run in buffer.layout_runs() {
            let mut line = PreparedLine {
                top: i32::MAX,
                bottom: i32::MIN,
                glyphs: Vec::new(),
            };
            for glyph in run.glyphs {
                let physical = glyph.physical((0., 0.), 1.);
                let Some(image) = self
                    .swash_cache
                    .get_image(&mut self.font_system, physical.cache_key)
                else {
                    continue;
                };
                let y = run.line_y as i32 + physical.y;
                let top = y - image.placement.top;
                line.top = line.top.min(top);
                line.bottom = line.bottom.max(top + image.placement.height as i32);
                line.glyphs.push(PreparedGlyph {
                    key: physical.cache_key,
                    x: physical.x,
                    y,
                    color: glyph.color_opt.unwrap_or(Color::rgb(255, 255, 255)),
                });
            }
            if !line.glyphs.is_empty() {
                lines.push(line);
            }
        }

        PreparedText { lines }
    }

    pub fn paint_text(
        &mut self,
        x: f32,
        y: f32,
        text: &PreparedText,
        pixmap: &mut Pixmap,
        strip_y: u32,
        canvas_height: u32,
    ) {
        let origin_x = x * self.scale_factor;
        let origin_y = y * self.scale_factor;
        assert!(
            origin_x.is_finite() && origin_y.is_finite(),
            "Cannot draw text at a non-finite origin"
        );
        let width = pixmap.width() as usize;
        let height = pixmap.height() as usize;
        let strip_bottom = u64::from(strip_y) + height as u64;
        let pixels = pixmap.data_mut();

        for line in &text.lines {
            if line.bottom as f32 + origin_y + 1. <= strip_y as f32
                || line.top as f32 + origin_y - 1. >= strip_bottom as f32
            {
                continue;
            }
            for glyph in &line.glyphs {
                self.swash_cache.with_pixels(
                    &mut self.font_system,
                    glyph.key,
                    glyph.color,
                    |glyph_x, glyph_y, color| {
                        let font_x = glyph.x + glyph_x;
                        let font_y = glyph.y + glyph_y;
                        // Avoid fill_rect's per-sample traversal of all tiles above 8191px
                        if color.a() == 0 {
                            return;
                        }

                        let x = font_x as f32 + origin_x;
                        let y = font_y as f32 + origin_y;
                        let left = x.max(0.);
                        let top = y.max(0.);
                        let right = (x + 1.).min(width as f32);
                        let bottom = (y + 1.).min(canvas_height as f32);
                        if left >= right || top >= bottom {
                            return;
                        }

                        // Match tiny-skia's rectangle AA: round 16.16 coordinates to
                        // 24.8 coverage units. Wider integers also work on tall images
                        let fixed = |value: f32| ((value * 65536.) as i64 + 128) >> 8;
                        let left = fixed(left).clamp(0, width as i64 * 256);
                        let top = fixed(top).clamp(0, i64::from(canvas_height) * 256);
                        let right = fixed(right).clamp(0, width as i64 * 256);
                        let bottom = fixed(bottom).clamp(0, i64::from(canvas_height) * 256);
                        if left >= right || top >= bottom {
                            return;
                        }

                        let source = ColorU8::from_rgba(color.r(), color.g(), color.b(), color.a())
                            .premultiply();
                        let source = [source.red(), source.green(), source.blue(), source.alpha()]
                            .map(u16::from);
                        let first_row = top >> 8;
                        let last_row = (bottom - 1) >> 8;

                        // Preserve sample order so overlapping glyphs blend identically
                        for row in first_row..=last_row {
                            // Clip rows after computing coverage against the full canvas
                            if row < i64::from(strip_y) || row >= strip_bottom as i64 {
                                continue;
                            }
                            let vertical = if first_row == last_row {
                                bottom - top - 1
                            } else {
                                bottom.min((row + 1) * 256) - top.max(row * 256)
                            };
                            for column in (left >> 8)..=((right - 1) >> 8) {
                                let horizontal =
                                    right.min((column + 1) * 256) - left.max(column * 256);
                                let coverage = ((vertical * horizontal) >> 8) as u16;
                                let index = ((row as usize - strip_y as usize) * width
                                    + column as usize)
                                    * 4;
                                let destination = &mut pixels[index..index + 4];

                                if source[3] == 255 {
                                    // Opaque paints use tiny-skia's coverage lerp
                                    for (destination, source) in destination.iter_mut().zip(source)
                                    {
                                        *destination = ((source * coverage
                                            + u16::from(*destination) * (255 - coverage)
                                            + 255)
                                            >> 8)
                                            as u8;
                                    }
                                } else {
                                    // Match tiny-skia's premultiplied SourceOver rounding
                                    let source =
                                        source.map(|channel| (channel * coverage + 255) >> 8);
                                    let inverse_alpha = 255 - source[3];
                                    for (destination, source) in destination.iter_mut().zip(source)
                                    {
                                        *destination = (source
                                            + ((u16::from(*destination) * inverse_alpha + 255)
                                                >> 8))
                                            as u8;
                                    }
                                }
                            }
                        }
                    },
                );
            }
        }
    }
}
