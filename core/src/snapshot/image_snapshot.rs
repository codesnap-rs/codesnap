use std::{
    collections::HashMap,
    io::Write,
    sync::{Arc, Mutex},
};

use crate::{
    components::{
        command_line::{
            command_line_header::CommandLineHeader, command_line_output::CommandLineOutput,
        },
        image::Image,
        interface::component::Component,
        layout::{column::Column, row::Row},
    },
    config::{self, CommandLineContent, SnapshotConfig, DEFAULT_WINDOW_MARGIN},
    rendering::Scene,
    utils::{color::RgbaColor, text::FontRenderer, theme_provider::ThemeProvider},
};
use tiny_skia::Color;

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
use base64::{engine::general_purpose::STANDARD, write::EncoderWriter};

use super::{png, snapshot_data::SnapshotData};

const DEFAULT_WINDOW_MIN_WIDTH: f32 = 350.;

pub struct ImageSnapshot {
    scene: Scene,
}

impl ImageSnapshot {
    pub fn raw_data(&self) -> Result<SnapshotData, anyhow::Error> {
        let pixmap = self.scene.render()?;

        Ok(SnapshotData::Image {
            width: pixmap.width() as usize,
            height: pixmap.height() as usize,
            data: pixmap.take(),
        })
    }

    pub fn png_data(&self) -> Result<SnapshotData, anyhow::Error> {
        let mut data = Vec::new();
        self.scene.write_png(&mut data)?;

        Ok(SnapshotData::Image {
            width: self.scene.width() as usize,
            height: self.scene.height() as usize,
            data,
        })
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
        let mut output = EncoderWriter::new(Vec::new(), &STANDARD);
        self.scene.write_png(&mut output)?;

        Ok(String::from_utf8(output.finish()?).unwrap())
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

    pub fn draw_code_content(
        window_padding: &Padding,
        code_content: config::Code,
    ) -> anyhow::Result<Vec<Box<dyn Component>>> {
        let code_lines = code_content.content.lines().collect::<Vec<&str>>();
        let view: Vec<Box<dyn Component>> = vec![
            Box::new(Breadcrumbs::from(code_content.file_path.clone())),
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
        let scene = Scene::from_config(config)?;

        Ok(Self { scene })
    }
}

impl Scene {
    fn from_config(config: SnapshotConfig) -> anyhow::Result<Self> {
        let theme_provider = ThemeProvider::from_config(&config)?;
        let window_padding = Padding {
            top: if config.window.mac_window_bar {
                14.
            } else {
                12.
            },
            ..Padding::from_value(14.)
        };
        let editor_background_color = theme_provider.theme_background();
        let border_rgba_color: RgbaColor = config.window.border.color.as_str().into();
        let shadow_color: RgbaColor = config.window.shadow.color.as_str().into();

        let render_content = match &config.content {
            crate::config::Content::Code(code) => {
                ImageSnapshot::draw_code_content(&window_padding, code.clone())?
            }
            crate::config::Content::CommandOutput(command_line_content) => {
                ImageSnapshot::command_line_content(command_line_content.clone())
            }
            crate::config::Content::Image(image_data) => {
                let image = Box::new(
                    Rect::new(
                        config.window.radius,
                        Color::from_rgba8(255, 255, 255, 0),
                        None,
                        Padding::default(),
                        "ImageContainer",
                        vec![Box::new(Image::new(
                            config.window.radius,
                            image_data.to_owned(),
                        )?)],
                    )
                    .shadow(
                        0.,
                        21.,
                        config.window.shadow.radius,
                        Color::from(shadow_color),
                    ),
                );

                return Self::with_frame(config, theme_provider, image);
            }
        };
        let mut children: Vec<Box<dyn Component>> = vec![Box::new(Row::from_children(vec![
            Box::new(MacTitleBar::new(config.window.mac_window_bar)),
            Box::new(Title::from_content(config.title.clone())),
        ]))];
        children.extend(render_content);
        let window = Rect::create_with_border(
            config.window.radius,
            editor_background_color.into(),
            DEFAULT_WINDOW_MIN_WIDTH,
            window_padding,
            config.window.border.width,
            border_rgba_color.into(),
            children,
        )
        .shadow(
            0.,
            21.,
            config.window.shadow.radius,
            Color::from(shadow_color),
        );

        Self::with_frame(config, theme_provider, Box::new(window))
    }

    fn with_frame(
        config: SnapshotConfig,
        theme_provider: ThemeProvider,
        render_content: Box<dyn Component>,
    ) -> anyhow::Result<Self> {
        let background_padding = Padding::from(config.window.margin.clone());

        // If vertical background padding is less than 82., should hidden watermark component
        // If watermark text is equal to "", the watermark component is hidden
        let watermark = if background_padding.bottom >= DEFAULT_WINDOW_MARGIN {
            config.watermark.clone()
        } else {
            None
        };
        let mut context = ComponentContext {
            scale_factor: config.scale_factor as f32,
            font_renderer: Arc::new(Mutex::new(FontRenderer::new(
                config.scale_factor as f32,
                config.fonts_folders.clone(),
            ))),
            take_snapshot_params: config,
            theme_provider,
            style_map: HashMap::new(),
        };

        // Draw the image snapshot frame template
        let scene = Container::from_children(vec![Box::new(Background::new(
            background_padding,
            vec![render_content, Box::new(Watermark::new(watermark))],
        ))])
        .prepare_scene(&mut context)?;

        Ok(scene)
    }

    fn write_png(&self, output: impl Write) -> anyhow::Result<()> {
        let (width, height) = (self.width(), self.height());
        let strips = (0..height)
            .step_by(512)
            .map(|y| self.render_strip(y, (height - y).min(512)));

        png::write(output, width, height, strips)
    }
}

impl SnapshotConfig {
    /// Render a PNG in bounded-height strips without allocating a full RGBA canvas
    pub fn write_png(&self, output: impl Write) -> anyhow::Result<()> {
        Scene::from_config(self.clone())?.write_png(output)
    }
}
