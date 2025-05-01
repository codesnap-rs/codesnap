use cosmic_text::{
    Align, Attrs, AttrsList, Buffer, BufferLine, Color, FontSystem, LayoutRunIter, LineEnding,
    Metrics, Shaping, SwashCache,
};
use tiny_skia::{Paint, Pixmap, Rect, Transform};

const CASKAYDIA_COVE_NERD_FONT: &[u8] =
    include_bytes!("../../assets/fonts/CaskaydiaCoveNerdFont-Regular.ttf");
const PACIFICO_FONT: &[u8] = include_bytes!("../../assets/fonts/Pacifico-Regular.ttf");

pub struct FontRenderer {
    font_system: FontSystem,
    scale_factor: f32,
}

impl FontRenderer {
    pub fn new(scale_factor: f32, fonts_folders: Vec<String>) -> FontRenderer {
        let mut font_system = FontSystem::new();
        let font_db = font_system.db_mut();

        font_db.load_system_fonts();
        font_db.load_font_data(PACIFICO_FONT.into());
        font_db.load_font_data(CASKAYDIA_COVE_NERD_FONT.into());

        for folder in fonts_folders {
            font_db.load_fonts_dir(folder);
        }

        FontRenderer {
            font_system,
            scale_factor,
        }
    }

    pub fn measure_text(&mut self, metrics: Metrics, text: &str) -> (f32, f32) {
        let mut buffer = Buffer::new(&mut self.font_system, metrics.scale(self.scale_factor));

        buffer.set_text(&mut self.font_system, text, Attrs::new(), Shaping::Advanced);

        let layout_runs: LayoutRunIter = buffer.layout_runs();
        let line_height = buffer.lines.len() as f32 * buffer.metrics().line_height;
        let mut_width = layout_runs.fold(0f32, |max_width, run| max_width.max(run.line_w));

        (
            mut_width.ceil() / self.scale_factor,
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
        let mut buffer = Buffer::new(&mut self.font_system, metrics.scale(self.scale_factor));

        buffer.set_rich_text(
            &mut self.font_system,
            spans,
            Attrs::new(),
            Shaping::Advanced,
        );

        self.draw(x, y, &mut buffer, pixmap);
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
        let mut buffer = Buffer::new(&mut self.font_system, metrics.scale(self.scale_factor));
        let mut line = if cfg!(windows) {
            BufferLine::new(
                line,
                LineEnding::CrLf,
                AttrsList::new(attrs),
                Shaping::Advanced,
            )
        } else {
            BufferLine::new(
                line,
                LineEnding::Lf,
                AttrsList::new(attrs),
                Shaping::Advanced,
            )
        };

        line.set_align(align);
        buffer.lines = vec![line];
        buffer.set_size(
            &mut self.font_system,
            Some(pixmap.width() as f32),
            Some(pixmap.height() as f32),
        );
        self.draw(x, y, &mut buffer, pixmap);
    }

    fn draw<'a>(&mut self, x: f32, y: f32, buffer: &mut Buffer, pixmap: &mut Pixmap) {
        let mut swash_cache = SwashCache::new();
        let default_font_color = Color::rgb(255, 255, 255);

        buffer.draw(
            &mut self.font_system,
            &mut swash_cache,
            default_font_color,
            |font_x, font_y, w, h, color| {
                let mut paint = Paint {
                    anti_alias: true,
                    ..Default::default()
                };

                paint.set_color_rgba8(color.r(), color.g(), color.b(), color.a());

                let rect = Rect::from_xywh(
                    font_x as f32 + x * self.scale_factor,
                    font_y as f32 + y * self.scale_factor,
                    w as f32,
                    h as f32,
                )
                .expect("Cannot draw text on pixmap");

                pixmap.fill_rect(rect, &paint, Transform::identity(), None);
            },
        );
    }
}
