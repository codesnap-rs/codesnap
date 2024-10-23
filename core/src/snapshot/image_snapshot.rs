use std::{sync::Arc, thread};

use crate::{config::SnapshotConfig, utils::path::parse_save_path};
use anyhow::bail;
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
use arboard::{Clipboard, ImageData};
use tiny_skia::{Color, Pixmap};

use crate::{
    components::{
        background::Background,
        breadcrumbs::Breadcrumbs,
        code_block::CodeBlock,
        container::Container,
        editor::{code::Code, mac_title_bar::MacTitleBar, title::Title},
        highlight_code_block::HighlightCodeBlock,
        interface::component::ComponentContext,
        line_number::LineNumber,
        rect::Rect,
        row::Row,
        watermark::Watermark,
    },
    edges::padding::Padding,
};

use super::snapshot::Snapshot;

const SCALE_FACTOR: f32 = 3.;
const LINE_HEIGHT: f32 = 18.;
const VIEW_WATERMARK_PADDING: f32 = 82.;

pub struct ImageSnapshot {
    pixmap: Pixmap,
}

impl Snapshot for ImageSnapshot {
    fn copy(&self) -> anyhow::Result<()> {
        let colors = self.pixmap.data();
        let image_data = ImageData {
            width: self.pixmap.width() as usize,
            height: self.pixmap.height() as usize,
            bytes: colors.into(),
        };

        #[cfg(target_os = "linux")]
        thread::scope(|s| -> Result<(), arboard::Error> {
            s.spawn(|| -> Result<(), arboard::Error> {
                Clipboard::new().unwrap().set().wait().image(image_data)
            })
            .join()
            .unwrap()
        })?;

        #[cfg(not(target_os = "linux"))]
        Clipboard::new().unwrap().set_image(image_data)?;

        Ok(())
    }

    fn save(&self, save_path: &str) -> anyhow::Result<()> {
        if !save_path.ends_with(".png") {
            bail!("The save_path must ends with .png");
        }

        let path = parse_save_path(save_path.to_string())?;

        self.pixmap.save_png(path)?;

        Ok(())
    }

    fn from_config(config: SnapshotConfig) -> anyhow::Result<ImageSnapshot> {
        let context = ComponentContext {
            scale_factor: SCALE_FACTOR,
            take_snapshot_params: Arc::new(config.clone()),
        };
        let background_padding =
            Padding::from_config(config.bg_x_padding, config.bg_y_padding, config.bg_padding);

        // If vertical background padding is less than 82., should hidden watermark component
        // If watermark text is equal to "", the watermark component is hidden
        let watermark = if background_padding.bottom >= VIEW_WATERMARK_PADDING {
            config.watermark.clone()
        } else {
            None
        };
        let pixmap = Container::from_children(vec![Box::new(Background::new(
            background_padding,
            vec![
                Box::new(
                    Rect::create_with_border(
                        12.,
                        Color::from_rgba8(40, 44, 52, 229),
                        config.min_width,
                        Padding::from_value(16.),
                        1.,
                        Color::from_rgba8(255, 255, 255, 50),
                        vec![
                            Box::new(Row::from_children(vec![
                                Box::new(MacTitleBar::from_radius(6., config.mac_window_bar)),
                                Box::new(Title::from_text(config.title)),
                            ])),
                            Box::new(Breadcrumbs::from_path(
                                config.file_path.clone(),
                                15.,
                                config.breadcrumbs_separator.clone(),
                                config.has_breadcrumbs,
                            )),
                            Box::new(CodeBlock::from_children(vec![
                                Box::new(HighlightCodeBlock::from_line_number(
                                    config.highlight_start_line_number,
                                    config.highlight_end_line_number,
                                    LINE_HEIGHT,
                                )),
                                Box::new(LineNumber::new(
                                    &config.code,
                                    config.start_line_number,
                                    LINE_HEIGHT,
                                )),
                                Box::new(Code::new(&config.code, LINE_HEIGHT, 12.5)),
                            ])),
                        ],
                    )
                    .shadow(0., 21., 20., Color::from_rgba8(0, 0, 0, 80)),
                ),
                Box::new(Watermark::new(watermark)),
            ],
        ))])
        .draw_root(&context)?;

        Ok(ImageSnapshot { pixmap })
    }
}
