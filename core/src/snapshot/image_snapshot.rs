use std::sync::Arc;

use crate::{
    config::{SnapshotConfig, VIEW_WATERMARK_PADDING},
    utils::{clipboard::Clipboard, path::parse_file_name, theme_provider::ThemeProvider},
};
use anyhow::bail;
use arboard::ImageData;
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
use base64::{engine::general_purpose::STANDARD, Engine as _};

use super::snapshot::Snapshot;

const LINE_HEIGHT: f32 = 18.;

pub struct ImageSnapshot {
    pixmap: Pixmap,
    svg_content: Option<String>,
}

impl Snapshot for ImageSnapshot {
    /// Copy the code snapshot to the clipboard
    fn copy(&self) -> anyhow::Result<()> {
        let mut clipboard = Clipboard::new()?;

        match &self.svg_content {
            Some(svg_content) => clipboard.set_text(svg_content)?,
            None => {
                let colors = self.pixmap.data();
                let image_data = ImageData {
                    width: self.pixmap.width() as usize,
                    height: self.pixmap.height() as usize,
                    bytes: colors.into(),
                };

                clipboard.set_image(image_data)?;
            }
        }

        Ok(())
    }

    /// Save the code snapshot to disk as PNG, please make sure you have set the `save_png`
    /// before calling this method
    fn save(&self, save_path: &str) -> anyhow::Result<()> {
        if !save_path.ends_with(".png") || !save_path.ends_with(".svg") {
            bail!("The save_path must ends with .png or .svg");
        }

        let path = parse_file_name(save_path)?;

        self.pixmap.save_png(path)?;

        Ok(())
    }

    fn from_config(config: SnapshotConfig) -> anyhow::Result<Self> {
        let theme_provider = ThemeProvider::from(
            config.themes_folder.clone(),
            &config.theme,
            config.language.clone(),
            config.code_file_path.clone(),
            &config.code,
        )?;
        let editor_background_color = theme_provider.theme_background();
        let context = ComponentContext {
            scale_factor: config.scale_factor as f32,
            take_snapshot_params: Arc::new(config.clone()),
            theme_provider,
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
                        editor_background_color.into(),
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

        Ok(ImageSnapshot {
            pixmap,
            svg_content: None,
        })
    }
}

impl ImageSnapshot {
    /// CodeSnap use tiny_skia to generate the image snapshot, and the format of generated image
    /// is PNG, if you want a SVG code snapshot, you can use this method to convert the PNG to SVG
    ///
    /// WARNING: This method is not really convert the PNG to SVG, it encode PNG to Base64 and
    /// format it to SVG, so the SVG file is still a image file, not a real SVG file. And the
    /// encoding process will take some time, which depends on the size of the PNG image, if
    /// the PNG image is too large, the encoding process will take a long time.
    pub fn to_svg(&mut self) -> Result<&Self, anyhow::Error> {
        let png_data = self.pixmap.encode_png()?;
        let encoded_base64_png_data = STANDARD.encode(png_data);
        let parsed_svg_content = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><image href="data:image/png;base64,{}"/></svg>"#,
            encoded_base64_png_data.as_str()
        );

        self.svg_content = Some(parsed_svg_content);
        Ok(self)
    }
}
