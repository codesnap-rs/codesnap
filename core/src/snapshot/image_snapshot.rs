use std::sync::Arc;

use crate::{
    config::{Border, SnapshotConfig, DEFAULT_WINDOW_MARGIN},
    utils::{
        clipboard::Clipboard, color::RgbaColor, path::parse_file_name,
        theme_provider::ThemeProvider,
    },
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
const DEFAULT_WINDOW_MIN_WIDTH: f32 = 350.;

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
        if !save_path.ends_with(".png") && !save_path.ends_with(".svg") {
            bail!("The save_path must ends with .png or .svg");
        }

        let path = parse_file_name(save_path)?;

        self.pixmap.save_png(path)?;

        Ok(())
    }
}

impl ImageSnapshot {
    /// CodeSnap use tiny_skia to generate the image snapshot, and the format of generated image
    /// is PNG, if you want a SVG code snapshot, you can use this method to convert the PNG to SVG
    ///
    /// WARNING: This method is not really convert the PNG to SVG, it encode PNG to Base64 and
    /// format it to SVG, so the SVG file is still a image file, not a real SVG file. Base64
    /// usually takes about 33% more space than the original data, so the SVG file size might be larger.
    pub fn to_svg(mut self) -> Result<impl Snapshot, anyhow::Error> {
        let png_data = self.pixmap.encode_png()?;
        let encoded_base64_png_data = STANDARD.encode(png_data);
        let parsed_svg_content = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><image href="data:image/png;base64,{}"/></svg>"#,
            encoded_base64_png_data.as_str()
        );

        self.svg_content = Some(parsed_svg_content);
        Ok(self)
    }

    pub fn from_config(config: SnapshotConfig) -> anyhow::Result<Self> {
        let theme_provider = ThemeProvider::from(
            config.themes_folder.clone(),
            &config.code.theme,
            config.code.language.clone(),
            config.code.file_path.clone(),
            &config.code.content,
        )?;
        let editor_background_color = theme_provider.theme_background();
        let context = ComponentContext {
            scale_factor: config.scale_factor as f32,
            take_snapshot_params: Arc::new(config.clone()),
            theme_provider,
        };
        let code_lines = config.code.content.lines().collect::<Vec<&str>>();
        let background_padding = Padding::from(config.window.margin);

        // If vertical background padding is less than 82., should hidden watermark component
        // If watermark text is equal to "", the watermark component is hidden
        let watermark = if background_padding.bottom >= DEFAULT_WINDOW_MARGIN {
            config.watermark.clone()
        } else {
            None
        };
        let window_padding = Padding {
            top: if config.window.mac_window_bar {
                14.
            } else {
                12.
            },
            ..Padding::from_value(14.)
        };
        // CodeSnap not support custom border width for now
        let border_width = match config.window.border {
            Some(_) => 1.,
            None => 0.,
        };
        let border_rgba_color: RgbaColor = config
            .window
            .border
            .unwrap_or(Border {
                color: String::from("#ffffff30"),
            })
            .color
            .as_str()
            .into();
        let pixmap = Container::from_children(vec![Box::new(Background::new(
            background_padding,
            vec![
                Box::new(
                    Rect::create_with_border(
                        12.,
                        editor_background_color.into(),
                        DEFAULT_WINDOW_MIN_WIDTH,
                        window_padding.clone(),
                        border_width,
                        border_rgba_color.into(),
                        vec![
                            Box::new(Row::from_children(vec![
                                Box::new(MacTitleBar::new(config.window.mac_window_bar)),
                                Box::new(Title::from_config(config.window.title)),
                            ])),
                            Box::new(Breadcrumbs::from_path(
                                config.code.file_path.clone(),
                                config.code.breadcrumbs.clone(),
                            )),
                            Box::new(CodeBlock::from_children(vec![
                                Box::new(HighlightCodeBlock::from(
                                    config.code.highlight_lines.clone(),
                                    code_lines.len(),
                                    LINE_HEIGHT,
                                    window_padding,
                                )),
                                Box::new(LineNumber::new(config.code.clone(), LINE_HEIGHT)),
                                Box::new(Code::new(&config.code.content, LINE_HEIGHT, 12.5)),
                            ])),
                        ],
                    )
                    .shadow(
                        0.,
                        21.,
                        config.window.shadow,
                        Color::from_rgba8(0, 0, 0, 80),
                    ),
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
