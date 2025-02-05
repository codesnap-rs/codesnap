use std::sync::{Arc, Mutex};

use crate::{
    components::{
        command_line::{
            command_line_header::CommandLineHeader, command_line_output::CommandLineOutput,
        },
        interface::component::Component,
        layout::{column::Column, row::Row},
    },
    config::{self, CommandLineContent, SnapshotConfig, DEFAULT_WINDOW_MARGIN},
    utils::{color::RgbaColor, text::FontRenderer, theme_provider::ThemeProvider},
};
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
        watermark::Watermark,
    },
    edges::padding::Padding,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};

use super::snapshot_data::SnapshotData;

const DEFAULT_WINDOW_MIN_WIDTH: f32 = 350.;

pub struct ImageSnapshot {
    pixmap: Pixmap,
}

impl ImageSnapshot {
    pub fn raw_data(&self) -> Result<SnapshotData, anyhow::Error> {
        Ok(SnapshotData::from_pixmap(&self.pixmap, false)?)
    }

    pub fn png_data(&self) -> Result<SnapshotData, anyhow::Error> {
        Ok(SnapshotData::from_pixmap(&self.pixmap, true)?)
    }

    pub fn svg_data(&self) -> Result<SnapshotData, anyhow::Error> {
        Ok(SnapshotData::Text(self.to_svg()?))
    }

    pub fn html_data(&self) -> Result<SnapshotData, anyhow::Error> {
        Ok(SnapshotData::Text(self.to_html()?))
    }
}

impl ImageSnapshot {
    pub fn to_html(&self) -> Result<String, anyhow::Error> {
        Ok(format!(
            r#"<img src="data:image/png;base64,{}" />"#,
            self.to_base64()?
        ))
    }

    pub fn to_base64(&self) -> Result<String, anyhow::Error> {
        let png_data = self.pixmap.encode_png()?;

        Ok(STANDARD.encode(png_data))
    }

    /// CodeSnap use tiny_skia to generate the image snapshot, and the format of generated image
    /// is PNG, if you want a SVG code snapshot, you can use this method to convert the PNG to SVG
    ///
    /// WARNING: This method is not really convert the PNG to SVG, it encode PNG to Base64 and
    /// format it to SVG, so the SVG file is still a image file, not a real SVG file. Base64
    /// usually takes about 33% more space than the original data, so the SVG file size might be larger.
    pub fn to_svg(&self) -> Result<String, anyhow::Error> {
        let parsed_svg_content = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><image href="data:image/png;base64,{}"/></svg>"#,
            self.to_base64()?.as_str()
        );

        Ok(parsed_svg_content)
    }

    pub fn create_drawer_with_frame(
        config: SnapshotConfig,
        theme_provider: ThemeProvider,
        window_padding: Padding,
    ) -> Box<dyn Fn(Vec<Box<dyn Component>>) -> anyhow::Result<Pixmap>> {
        Box::new(move |render_content| {
            let editor_background_color = theme_provider.theme_background();
            let font_renderer = Mutex::new(FontRenderer::new(
                config.scale_factor as f32,
                config.fonts_folder.clone().unwrap_or_default().as_str(),
            ));
            let context = ComponentContext {
                scale_factor: config.scale_factor as f32,
                take_snapshot_params: Arc::new(config.clone()),
                theme_provider: theme_provider.clone(),
                font_renderer,
            };
            let background_padding = Padding::from(config.window.margin.clone());
            let border_rgba_color: RgbaColor = config.window.border.color.as_str().into();

            // If vertical background padding is less than 82., should hidden watermark component
            // If watermark text is equal to "", the watermark component is hidden
            let watermark = if background_padding.bottom >= DEFAULT_WINDOW_MARGIN {
                config.watermark.clone()
            } else {
                None
            };

            let mut parsed_render_content: Vec<Box<dyn Component>> =
                vec![Box::new(Row::from_children(vec![
                    Box::new(MacTitleBar::new(config.window.mac_window_bar)),
                    Box::new(Title::from_content(config.title.clone())),
                ]))];

            parsed_render_content.extend(render_content);

            let shadow_color: RgbaColor = config.window.shadow.color.as_str().into();

            // Draw the image snapshot frame template
            let pixmap = Container::from_children(vec![Box::new(Background::new(
                background_padding,
                vec![
                    Box::new(
                        Rect::create_with_border(
                            12.,
                            editor_background_color.into(),
                            DEFAULT_WINDOW_MIN_WIDTH,
                            window_padding.clone(),
                            config.window.border.width,
                            border_rgba_color.into(),
                            parsed_render_content,
                        )
                        .shadow(
                            0.,
                            21.,
                            config.window.shadow.radius,
                            Color::from(shadow_color),
                        ),
                    ),
                    Box::new(Watermark::new(watermark)),
                ],
            ))])
            .draw_root(&context)?;

            Ok(pixmap)
        })
    }

    pub fn draw_code_content(
        window_padding: &Padding,
        code_content: config::Code,
    ) -> anyhow::Result<Vec<Box<dyn Component>>> {
        let code_lines = code_content.content.lines().collect::<Vec<&str>>();
        let view: Vec<Box<dyn Component>> = vec![
            Box::new(Breadcrumbs::from(
                code_content.has_breadcrumbs,
                code_content.file_path.clone(),
            )),
            Box::new(CodeBlock::from_children(vec![
                Box::new(HighlightCodeBlock::from(
                    code_content.highlight_lines.clone(),
                    code_lines.len(),
                    window_padding.clone(),
                )),
                Box::new(LineNumber::new(code_content.clone())),
                Box::new(Code::new(code_content.clone())?),
            ])),
        ];

        Ok(view)
    }

    pub fn command_line_content(
        command_line_content: Vec<CommandLineContent>,
    ) -> Vec<Box<dyn Component>> {
        command_line_content
            .clone()
            .into_iter()
            .map(|output| {
                Box::new(Column::from_children(vec![
                    Box::new(CommandLineHeader::from(&output.full_command)),
                    Box::new(CommandLineOutput::from(&output.content)),
                ])) as Box<dyn Component>
            })
            .collect::<Vec<Box<dyn Component>>>()
    }

    pub fn from_config(config: SnapshotConfig) -> anyhow::Result<Self> {
        let theme_provider = ThemeProvider::from_config(&config)?;
        let window_padding = Padding {
            top: if config.window.mac_window_bar {
                14.
            } else {
                12.
            },
            ..Padding::from_value(14.)
        };

        let drawer = Self::create_drawer_with_frame(
            config.clone(),
            theme_provider.clone(),
            window_padding.clone(),
        );
        let pixmap = match config.content {
            crate::config::Content::Code(code) => {
                drawer(Self::draw_code_content(&window_padding, code)?)
            }
            crate::config::Content::CommandOutput(command_line_content) => {
                drawer(Self::command_line_content(command_line_content))
            }
        }?;

        Ok(Self { pixmap })
    }
}
