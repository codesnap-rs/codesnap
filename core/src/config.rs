use derive_builder::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tiny_skia::{Color, GradientStop};

use crate::{
    snapshot::{ascii_snapshot::ASCIISnapshot, image_snapshot::ImageSnapshot},
    themes::get_theme,
    utils::color::RgbaColor,
};

pub const DEFAULT_WINDOW_MARGIN: f32 = 82.;

#[derive(Clone, Serialize, Debug, JsonSchema)]
#[serde(untagged)]
pub enum DimensionValue {
    Num(f32),
    Max,
}

impl<'de> Deserialize<'de> for DimensionValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum AnyType {
            Num(f32),
            Max(String),
        }

        Ok(match AnyType::deserialize(deserializer)? {
            AnyType::Num(num) => DimensionValue::Num(num),
            AnyType::Max(max) if max == "max" => DimensionValue::Max,
            _ => {
                return Err(serde::de::Error::custom(
                    "The value of DimensionValue should be a number or 'max'",
                ))
            }
        })
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub type GradientPoint = Point<DimensionValue>;

impl Point<DimensionValue> {
    pub fn into_f32_point(&self, pixmap_width: f32, pixmap_height: f32) -> Point<f32> {
        let x = match self.x {
            DimensionValue::Num(num) => num,
            DimensionValue::Max => pixmap_width,
        };
        let y = match self.y {
            DimensionValue::Num(num) => num,
            DimensionValue::Max => pixmap_height,
        };

        Point { x, y }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct LinearGradientStop {
    position: f32,
    color: String,
}

impl LinearGradientStop {
    pub fn new(position: f32, color: &str) -> Self {
        if position < 0. || position > 1. {
            panic!("The position of the gradient stop should be in the range of 0.0 to 1.0");
        }

        LinearGradientStop {
            position,
            color: color.to_string(),
        }
    }
}

impl From<LinearGradientStop> for GradientStop {
    fn from(stop: LinearGradientStop) -> Self {
        let rgba_color: RgbaColor = stop.color.as_str().into();
        let color: Color = rgba_color.into();

        GradientStop::new(stop.position, color)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct LinearGradient {
    pub start: GradientPoint,
    pub end: GradientPoint,
    pub stops: Vec<LinearGradientStop>,
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
#[serde(untagged)]
pub enum Background {
    Solid(String),
    Gradient(LinearGradient),
}

#[derive(Clone, Builder, Serialize, Deserialize, Debug, JsonSchema)]
pub struct TitleConfig {
    #[builder(setter(into, strip_option), default = String::from("CaskaydiaCove Nerd Font"))]
    pub font_family: String,

    #[builder(setter(into), default = String::from("#aca9b2"))]
    pub color: String,
}

#[derive(Clone, Builder, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Margin {
    #[builder(setter(into, strip_option), default = DEFAULT_WINDOW_MARGIN)]
    pub x: f32,

    #[builder(setter(into, strip_option), default = DEFAULT_WINDOW_MARGIN)]
    pub y: f32,
}

#[derive(Clone, Builder, Serialize, Deserialize, Debug, JsonSchema, Default)]
pub struct Breadcrumbs {
    #[builder(default = false)]
    pub enable: bool,

    #[builder(setter(into, strip_option), default = String::from("/"))]
    pub separator: String,

    #[builder(setter(into, strip_option), default = String::from("CaskaydiaCove Nerd Font"))]
    pub font_family: String,

    #[builder(setter(into), default = String::from("#80848b"))]
    pub color: String,
}

#[derive(Clone, Builder, Default, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Border {
    #[builder(setter(into), default = String::from("#ffffff30"))]
    pub color: String,

    #[builder(setter(into), default = 1.)]
    pub width: f32,
}

#[derive(Clone, Builder, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Shadow {
    #[builder(default = 20.)]
    pub radius: f32,

    #[builder(setter(into), default = String::from("#0000004d"))]
    pub color: String,
}

#[derive(Clone, Builder, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Window {
    #[builder(setter(into), default = MarginBuilder::default().build().unwrap())]
    pub margin: Margin,

    #[builder(setter(into), default = TitleConfigBuilder::default().build().unwrap())]
    pub title_config: TitleConfig,

    #[builder(setter(into), default = BorderBuilder::default().build().unwrap())]
    pub border: Border,

    #[builder(default = true)]
    pub mac_window_bar: bool,

    #[builder(default = ShadowBuilder::default().build().unwrap())]
    pub shadow: Shadow,
}

impl WindowBuilder {
    pub fn from_window(window: Window) -> WindowBuilder {
        WindowBuilder {
            margin: Some(window.margin),
            title_config: Some(window.title_config),
            border: Some(window.border),
            mac_window_bar: Some(window.mac_window_bar),
            shadow: Some(window.shadow),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
#[serde(untagged)]
pub enum HighlightLine {
    Single(u32, String),
    Range(u32, u32, String),
}

#[derive(Clone, Builder, Serialize, Deserialize, Debug, JsonSchema)]
pub struct CommandLineContent {
    #[builder(setter(into))]
    pub content: String,

    #[builder(setter(into))]
    pub full_command: String,
}

#[derive(Clone, Builder, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Code {
    #[builder(setter(into))]
    pub content: String,

    #[builder(setter(into, strip_option), default = None)]
    pub start_line_number: Option<u32>,

    #[builder(setter(into), default = vec![])]
    #[serde(default)]
    pub highlight_lines: Vec<HighlightLine>,

    /// The `language` will be used to determine the syntax highlighting to use for generating
    /// the snapshot.
    #[builder(setter(into, strip_option), default = None)]
    pub language: Option<String>,

    #[builder(setter(into, strip_option), default = None)]
    pub file_path: Option<String>,
}

#[derive(Clone, Builder, Serialize, Deserialize, Debug, JsonSchema, Default)]
pub struct CommandOutputConfig {
    #[builder(setter(into), default = String::from("❯"))]
    pub prompt: String,

    #[builder(setter(into), default = String::from("CaskaydiaCove Nerd Font"))]
    pub font_family: String,

    #[builder(setter(into), default = String::from("#F78FB3"))]
    pub prompt_color: String,

    #[builder(setter(into), default = String::from("#98C379"))]
    pub command_color: String,

    #[builder(setter(into), default = String::from("#ff0000"))]
    pub string_arg_color: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
#[serde(untagged)]
pub enum Content {
    Code(Code),
    CommandOutput(Vec<CommandLineContent>),
    Image(Vec<u8>),
}

#[derive(Clone, Builder, Serialize, Deserialize, Debug, JsonSchema, Default)]
pub struct CodeConfig {
    // #[builder(setter(into), default = String::from(""))]
    // #[serde(default)]
    // pub content: String,
    #[builder(setter(into), default = String::from("CaskaydiaCove Nerd Font"))]
    pub font_family: String,

    /// Breadcrumbs is a useful and unique feature of CodeSnap, it can help users to understand the
    /// code location in the project. If the `has_breadcrumbs` is true, CodeSnap will display the
    /// `file_path` on top of the code.
    ///
    /// The code snapshot is different from normal screenshots, it should provide more information
    /// about the code, such as the file path, the line number and highlight code line, these
    /// information can help users to understand the code better.
    #[builder(setter(into, strip_option), default = BreadcrumbsBuilder::default().build().unwrap())]
    #[serde(default)]
    pub breadcrumbs: Breadcrumbs,
}

/// Draw a watermark below the code, you can use this to add a logo or any other text
/// The watermark is designed as a place for users to provide personalize label
#[derive(Serialize, Deserialize, Clone, Builder, Debug, JsonSchema)]
pub struct Watermark {
    #[builder(setter(into))]
    pub content: String,

    #[builder(setter(into), default = String::from("Pacifico"))]
    pub font_family: String,

    #[builder(setter(into), default = String::from("#ffffff"))]
    pub color: String,
}

impl WatermarkBuilder {
    pub fn from_watermark(watermark: Option<Watermark>) -> WatermarkBuilder {
        watermark
            .and_then(|watermark| {
                Some(WatermarkBuilder {
                    content: Some(watermark.content),
                    font_family: Some(watermark.font_family),
                    color: Some(watermark.color),
                })
            })
            .unwrap_or(WatermarkBuilder::default())
    }
}

#[derive(Clone, Builder, Serialize, Deserialize, Debug, JsonSchema)]
#[builder(name = "CodeSnap", build_fn(validate = "Self::validate"))]
#[builder(derive(serde::Deserialize, serde::Serialize, Debug, JsonSchema))]
pub struct SnapshotConfig {
    #[builder(setter(into, strip_option), default = WindowBuilder::default().build().unwrap())]
    pub window: Window,

    /// The code to be displayed in the snapshot
    #[builder(setter(into), default = CommandOutputConfigBuilder::default().build().unwrap())]
    pub command_output_config: CommandOutputConfig,

    #[builder(setter(into), default = CodeConfigBuilder::default().build().unwrap())]
    pub code_config: CodeConfig,

    #[builder(setter(into), default = None)]
    pub watermark: Option<Watermark>,

    #[builder(setter(into))]
    pub content: Content,

    /// CodeSnap default generate triple size snapshot image,
    /// you can use this config to change the scale factor.
    #[builder(default = 3)]
    #[serde(default = "default_scale_factor")]
    pub scale_factor: u8,

    /// CodeSnap use Syntect as the syntax highlighting engine, you can provide a custom theme
    /// for the snapshot. If the `themes_folders` is provided, CodeSnap will load the theme from
    /// the folder, otherwise, CodeSnap will load the default themes.
    ///
    /// Visit https://github.com/trishume/syntect for more detail
    #[builder(setter(into, strip_option), default = vec![])]
    pub themes_folders: Vec<String>,

    /// Load fonts from the fonts_folders to render the code, CodeSnap use fonts which you have
    /// installed on your system by default, but you can still provide `fonts_folders` to tell
    /// CodeSnap to load extra fonts from the folder.
    ///
    /// This config is useful when you want to develop a tool based on CodeSnap, you can package
    /// some fonts with your tool and publish, so that users can use these fonts without installing
    /// them manually on their system.
    #[builder(setter(into, strip_option), default = vec![])]
    pub fonts_folders: Vec<String>,

    /// CodeSnap use Syntect as the syntax highlighting engine, you can provide a custom theme
    /// for code highlighting and background.
    /// The theme is load from the `themes_folders`(if not provided, CodeSnap load the default
    /// themes), you can use the theme name to specify the theme you want to use.
    ///
    /// See `themes_folders` config for more detail.
    #[builder(setter(into), default = String::from("candy"))]
    pub theme: String,

    #[builder(setter(into))]
    pub background: Background,

    #[builder(setter(into), default = String::from("#495162"))]
    pub line_number_color: String,

    #[builder(setter(into, strip_option), default = None)]
    pub title: Option<String>,
}

impl CodeSnap {
    fn validate(&self) -> Result<(), String> {
        if let Some(scale_factor) = self.scale_factor {
            if scale_factor < 1 {
                return Err("The scale factor must be greater than 1".to_string());
            }
        }

        Ok(())
    }

    pub fn from_default_theme() -> Result<CodeSnap, serde_json::Error> {
        Self::from_theme("bamboo")
    }

    pub fn from_theme(theme_name: &str) -> Result<CodeSnap, serde_json::Error> {
        let theme = get_theme(theme_name);

        Self::from_config(&theme)
    }

    pub fn from_config(config: &str) -> Result<CodeSnap, serde_json::Error> {
        serde_json::from_str::<CodeSnap>(config)
    }

    pub fn map_code_config<F>(&mut self, f: F) -> anyhow::Result<&mut Self>
    where
        F: Fn(CodeConfig) -> anyhow::Result<CodeConfig>,
    {
        self.code_config = Some(f(self
            .code_config
            .clone()
            .unwrap_or(CodeConfigBuilder::default().build()?))?);

        Ok(self)
    }

    pub fn map_content<F>(&mut self, f: F) -> anyhow::Result<&mut Self>
    where
        F: Fn(Code) -> anyhow::Result<Content>,
    {
        let content = self.content.clone().unwrap_or(Content::Code(
            CodeBuilder::default().content(String::from("")).build()?,
        ));
        let code_content = match content {
            Content::Code(code_content) => code_content,
            _ => return Ok(self),
        };

        self.content = Some(f(code_content)?);

        Ok(self)
    }

    pub fn map_window<F>(&mut self, f: F) -> anyhow::Result<&mut Self>
    where
        F: Fn(Window) -> anyhow::Result<Window>,
    {
        self.window = Some(f(self
            .window
            .clone()
            .unwrap_or(WindowBuilder::default().build()?))?);

        Ok(self)
    }

    pub fn map_watermark<F>(&mut self, f: F) -> anyhow::Result<&mut Self>
    where
        F: Fn(Option<Watermark>) -> anyhow::Result<Option<Watermark>>,
    {
        self.watermark = Some(f(self.watermark.clone().unwrap_or(None))?);

        Ok(self)
    }
}

impl SnapshotConfig {
    /// Create a beautiful code snapshot from the config
    pub fn create_snapshot(&self) -> anyhow::Result<ImageSnapshot> {
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

fn default_scale_factor() -> u8 {
    3
}
