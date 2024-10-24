use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::snapshot::{
    ascii_snapshot::ASCIISnapshot, image_snapshot::ImageSnapshot, snapshot::Snapshot,
};

pub const VIEW_WATERMARK_PADDING: f32 = 82.;

#[derive(Serialize, Deserialize, Clone, Builder)]
#[builder(name = "CodeSnap", build_fn(validate = "Self::validate"))]
pub struct SnapshotConfig {
    /// The code to be displayed in the snapshot
    #[builder(setter(into))]
    pub code: String,

    /// Draw a MacOS style window bar
    #[builder(default = true)]
    pub mac_window_bar: bool,

    /// Draw a watermark below the code, you can use this to add a logo or any other text
    /// The watermark is designed as a place for users to provide personalize label
    #[builder(setter(into, strip_option), default = None)]
    pub watermark: Option<String>,

    /// Config title of the code window
    #[builder(setter(into, strip_option), default = None)]
    pub title: Option<String>,

    /// Config font family you like to use for the code, default is "CaskaydiaCove Nerd Font"
    #[builder(setter(into), default = String::from("CaskaydiaCove Nerd Font"))]
    pub code_font_family: String,

    /// Config font family you like to use for the watermark, default is "Pacifico"
    #[builder(setter(into), default = String::from("Pacifico"))]
    pub watermark_font_family: String,

    /// The `code_file_path` will be read by Syntect, which will use the file content to detect which
    /// language highlighting to use for generating the snapshot.
    /// If the `language` field is provided, CodeSnap will prioritize the value provided by
    /// `language` for syntax highlighting.
    ///
    /// This config is useful for users who use CodeSnap editor plugins, in this case, the
    /// `code_file_path` should always have a value which from the editor.
    ///
    /// But if you want to generate a snapshot from a string of code, you should always use the
    /// `language` field to provide the language of the code or just let CodeSnap detect the
    /// language by itself.
    #[builder(setter(into, strip_option), default = None)]
    pub code_file_path: Option<String>,

    /// The `language` will be used to determine the syntax highlighting to use for generating
    /// the snapshot.
    #[builder(setter(into, strip_option), default = None)]
    pub language: Option<String>,

    /// Config the path where the snapshot will be saved, if the path is not provided, call `save`
    /// method will return an error.
    ///
    /// If you set `save_path` to a folder path, CodeSnap will save the snapshot to the folder with
    /// a random name, it's looks like `CodeSnap_1997-04-22_01:12:00.png` (or svg).
    ///
    /// For Linux and MacOS users:
    /// You can use `~` and `$HOME` in the path to represent the home directory, CodeSnap will
    /// replace them with the value of the `HOME` environment variable.
    #[builder(setter(into, strip_option), default = None)]
    pub save_path: Option<String>,

    /// CodeSnap use Syntect as the syntax highlighting engine, you can provide a custom theme
    /// for the snapshot. If the `themes_folder` is provided, CodeSnap will load the theme from
    /// the folder, otherwise, CodeSnap will load the default themes.
    ///
    /// Visit https://github.com/trishume/syntect for more detail
    #[builder(setter(into, strip_option), default = None)]
    pub themes_folder: Option<String>,

    /// Load fonts from the fonts_folder to render the code, CodeSnap use fonts which you have
    /// installed on your system by default, but you can still provide `fonts_folder` to tell
    /// CodeSnap to load extra fonts from the folder.
    ///
    /// This config is useful when you want to develop a tool based on CodeSnap, you can package
    /// some fonts with your tool and publish, so that users can use these fonts without installing
    /// them manually on their system.
    #[builder(setter(into, strip_option), default = None)]
    pub fonts_folder: Option<String>,

    /// CodeSnap use Syntect as the syntax highlighting engine, you can provide a custom theme
    /// for code highlighting and background.
    /// The theme is load from the `themes_folder`(if not provided, CodeSnap load the default
    /// themes), you can use the theme name to specify the theme you want to use.
    ///
    /// See `themes_folder` config for more detail.
    #[builder(setter(into), default = String::from("base16-ocean.dark"))]
    pub theme: String,

    /// CodeSnap have pre-defined background themes, you can use the theme name to specify the
    /// background theme you want to use.
    ///
    /// Currently available themes:
    /// - summer
    /// - bamboo
    /// - peach
    /// - grape
    /// - dusk
    /// - sea
    #[builder(setter(into), default = String::from("bamboo"))]
    pub bg_theme: String,

    /// Except for the pre-defined background themes, you can also provide a custom solid background
    /// the color should be a hex color string, e.g. "#ffffff"
    #[builder(setter(into, strip_option), default = None)]
    pub bg_color: Option<String>,

    /// The `file_path` will displayed in the snapshot as breadcrumbs
    #[builder(setter(into), default = String::from(""))]
    pub file_path: String,

    /// The separator of the breadcrumbs, default is "/"
    #[builder(setter(into), default = String::from("/"))]
    pub breadcrumbs_separator: String,

    /// Breadcrumbs is a useful and unique feature of CodeSnap, it can help users to understand the
    /// code location in the project. If the `has_breadcrumbs` is true, CodeSnap will display the
    /// `file_path` on top of the code.
    ///
    /// Also see the `file_path` config.
    ///
    /// The code snapshot is different from normal screenshots, it should provide more information
    /// about the code, such as the file path, the line number and highlight code line, these
    /// information can help users to understand the code better.
    #[builder(default = false)]
    pub has_breadcrumbs: bool,

    /// The `start_line_number` is used to specify the start line number of the code, if you use
    /// CodeSnap in editor plugins, the start line number will be the line number of the code in
    /// the editor.
    ///
    /// If the `start_line_number` is provided, CodeSnap will display the "line number" in the
    /// snapshot, otherwise, CodeSnap will not display the "line number".
    #[builder(setter(into, strip_option), default = None)]
    pub start_line_number: Option<u32>,

    /// CodeSnap can highlight multiple lines of code, to help users to understand the code better.
    /// The `highlight_start_line_number` specify the start line number of the highlight code.
    ///
    /// Please notice that the `highlight_start_line_number` and `highlight_end_line_number` should
    /// be provided together, and `highlight_start_line_number` should always <=
    /// `highlight_end_line_number`, otherwise, CodeSnap will throw a panic.
    ///
    /// Also see `highlight_end_line_number` config.
    #[builder(setter(into, strip_option), default = None)]
    pub highlight_start_line_number: Option<usize>,

    /// CodeSnap can highlight multiple lines of code, to help users to understand the code better.
    /// The `highlight_end_line_number` specify the end line number of the highlight code.
    ///
    /// Please notice that the `highlight_start_line_number` and `highlight_end_line_number` should
    /// be provided together, and `highlight_start_line_number` should always <=
    /// `highlight_end_line_number`, otherwise, CodeSnap will throw a panic.
    ///
    /// Also see `highlight_start_line_number` config.
    #[builder(setter(into, strip_option), default = None)]
    pub highlight_end_line_number: Option<usize>,

    /// The `min_width` is used to specify the minimum width of the code window, default is 350.
    #[builder(setter(into, strip_option), default = Some(350.))]
    pub min_width: Option<f32>,

    /// The `bg_x_padding` is used to specify the horizontal padding of the background
    /// default is `82`. If you want to set the same padding for both horizontal and vertical,
    /// you can use the `bg_padding` config.
    #[builder(default = VIEW_WATERMARK_PADDING)]
    pub bg_x_padding: f32,

    /// The `bg_y_padding` is used to specify the vertcal padding of the background
    /// default is `82`. If you want to set the same padding for both horizontal and vertical,
    /// you can use the `bg_padding` config.
    ///
    /// Please notice that if the `bg_y_padding` is less than 82., the watermark will be hidden.
    #[builder(default = VIEW_WATERMARK_PADDING)]
    pub bg_y_padding: f32,

    /// The `bg_padding` is used to specify same padding for both horizontal and vertical of the
    /// background, if you want to set different padding for horizontal and vertical, you can use
    /// the `bg_x_padding` and `bg_y_padding` config.
    #[builder(setter(into, strip_option), default = None)]
    pub bg_padding: Option<f32>,

    /// CodeSnap generate code snapshot image with scale `3.0` by default, you can use this config to
    /// specify the scale factor of the snapshot
    #[builder(default = 3)]
    pub scale_factor: u8,
}

impl CodeSnap {
    fn validate(&self) -> Result<(), String> {
        if let Some(scale_factor) = self.scale_factor {
            return match scale_factor {
                scale_factor if scale_factor < 1 => {
                    Err("The scale factor must be greater than 1".to_string())
                }
                _ => Ok(()),
            };
        }

        Ok(())
    }
}

impl SnapshotConfig {
    /// Create a beautiful code snapshot from the config
    pub fn create_snapshot(&self) -> anyhow::Result<ImageSnapshot, anyhow::Error> {
        ImageSnapshot::from_config(self.clone())
    }

    /// Create a ASCII "snapshot" from the config, the ASCII "snapshot" is a text representation of
    /// the code, it's useful when you want to display the code in the terminal or other text-based
    /// environment, and because of it's text-based, you can easily copy and paste it to anywhere.
    ///
    /// Through the ASCII "snapshot" is text-based, but it still has some basic styles, and it's
    /// will take some important information of code, such as the `line number` and `file path`,
    /// these information can help users to understand the code better.
    ///
    /// And If you want to highlighting the ASCII "snapshot", you can try put it into a markdown
    /// code block, most markdown renderers will highlight the code block for you.
    ///
    /// The ASCII "snapshot" is really cool, hope you like it!
    pub fn create_ascii_snapshot(&self) -> anyhow::Result<ASCIISnapshot> {
        ASCIISnapshot::from_config(self.clone())
    }
}
