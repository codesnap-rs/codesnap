use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tiny_skia::{Color, Pixmap};

use crate::components::background::Background;
use crate::components::breadcrumbs::Breadcrumbs;
use crate::components::code_block::CodeBlock;
use crate::components::container::Container;
use crate::components::editor::code::Code;
use crate::components::editor::mac_title_bar::MacTitleBar;
use crate::components::highlight_code_block::HighlightCodeBlock;
use crate::components::interface::component::ComponentContext;
use crate::components::interface::render_error;
use crate::components::line_number::LineNumber;
use crate::components::rect::Rect;
use crate::components::watermark::Watermark;
use crate::edges::padding::Padding;

#[derive(Serialize, Deserialize, Clone)]
pub struct SnapshotConfig {
    // Display the MacOS style title bar or not
    pub mac_window_bar: bool,
    // Wartermark of the code snapshot
    pub watermark: String,
    // Editor title
    pub title: Option<String>,
    pub code_font_family: String,
    pub watermark_font_family: String,
    pub code: String,
    pub code_file_path: String,
    pub extension: Option<String>,
    pub save_path: Option<String>,
    pub themes_folder: String,
    pub fonts_folder: String,
    pub theme: String,
    pub bg_theme: String,
    pub bg_color: Option<String>,
    // Breadcrumbs path
    pub file_path: String,
    pub breadcrumbs_separator: String,
    pub has_breadcrumbs: bool,
    pub start_line_number: Option<usize>,
    pub highlight_start_line_number: Option<usize>,
    pub highlight_end_line_number: Option<usize>,
    pub min_width: Option<f32>,
    pub bg_x_padding: f32,
    pub bg_y_padding: f32,
    pub bg_padding: Option<f32>,
}

// Scale the screenshot to 3 times its size
const SCALE_FACTOR: f32 = 3.;
const LINE_HEIGHT: f32 = 18.;
const VIEW_WATERMARK_PADDING: f32 = 82.;

// The params is come from neovim instance
pub fn create_template(config: SnapshotConfig) -> render_error::Result<Pixmap> {
    let context = ComponentContext {
        scale_factor: SCALE_FACTOR,
        take_snapshot_params: Arc::new(config.clone()),
    };
    let background_padding =
        Padding::from_config(config.bg_x_padding, config.bg_y_padding, config.bg_padding);

    // If vertical background padding is less than 82., should hidden watermark component
    // If watermark text is equal to "", the watermark component is hidden
    let watermark = if background_padding.bottom >= VIEW_WATERMARK_PADDING {
        config.watermark
    } else {
        "".to_string()
    };
    let pixmap = Container::from_children(vec![Box::new(Background::new(
        background_padding,
        vec![
            Box::new(
                Rect::create_with_border(
                    8.,
                    Color::from_rgba8(40, 44, 52, 229),
                    config.min_width,
                    Padding::from_value(16.),
                    1.,
                    Color::from_rgba8(255, 255, 255, 50),
                    vec![
                        Box::new(MacTitleBar::from_radius(6., config.mac_window_bar)),
                        Box::new(Breadcrumbs::from_path(
                            config.file_path,
                            15.,
                            config.breadcrumbs_separator,
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
                            Box::new(Code::new(config.code, LINE_HEIGHT, 12.5)),
                        ])),
                    ],
                )
                .shadow(0., 21., 20., Color::from_rgba8(0, 0, 0, 80)),
            ),
            Box::new(Watermark::new(watermark)),
        ],
    ))])
    .draw_root(&context)?;

    Ok(pixmap)
}
