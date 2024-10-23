use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::snapshot::{
    ascii_snapshot::ASCIISnapshot, image_snapshot::ImageSnapshot, snapshot::Snapshot,
};

#[derive(Serialize, Deserialize, Clone, Builder)]
#[builder(name = "CodeSnap")]
pub struct SnapshotConfig {
    #[builder(setter(into))]
    pub code: String,

    // Display the MacOS style title bar or not
    #[builder(default = true)]
    pub mac_window_bar: bool,

    // Wartermark of the code snapshot
    #[builder(setter(into, strip_option), default = None)]
    pub watermark: Option<String>,

    // Editor title
    #[builder(setter(into, strip_option), default = None)]
    pub title: Option<String>,

    #[builder(default = String::from("CaskaydiaCove Nerd Font"))]
    pub code_font_family: String,

    #[builder(default = String::from("Pacifico"))]
    pub watermark_font_family: String,

    #[builder(setter(into, strip_option), default = None)]
    pub code_file_path: Option<String>,

    #[builder(setter(into, strip_option), default = None)]
    pub language: Option<String>,

    #[builder(setter(into, strip_option), default = None)]
    pub save_path: Option<String>,

    #[builder(default = String::from("/Users/zhanhaozhao/repositories/codesnap/assets/themes"))]
    pub themes_folder: String,

    #[builder(default = String::from("/Users/zhanhaozhao/repositories/codesnap/assets/fonts"))]
    pub fonts_folder: String,

    #[builder(default = String::from("base16-onedark"))]
    pub theme: String,

    #[builder(default = String::from("bamboo"))]
    pub bg_theme: String,

    #[builder(setter(into, strip_option), default = None)]
    pub bg_color: Option<String>,

    #[builder(default = String::from(""))]
    pub file_path: String,

    #[builder(default = String::from("/"))]
    pub breadcrumbs_separator: String,

    #[builder(default = false)]
    pub has_breadcrumbs: bool,

    #[builder(setter(into, strip_option), default = None)]
    pub start_line_number: Option<u32>,

    #[builder(setter(into, strip_option), default = None)]
    pub highlight_start_line_number: Option<usize>,

    #[builder(setter(into, strip_option), default = None)]
    pub highlight_end_line_number: Option<usize>,

    #[builder(setter(into, strip_option), default = Some(350.))]
    pub min_width: Option<f32>,

    #[builder(default = 82.)]
    pub bg_x_padding: f32,

    #[builder(default = 82.)]
    pub bg_y_padding: f32,

    #[builder(setter(into, strip_option), default = None)]
    pub bg_padding: Option<f32>,
}

impl SnapshotConfig {
    pub fn create_snapshot(&self) -> anyhow::Result<impl Snapshot, anyhow::Error> {
        ImageSnapshot::from_config(self.clone())
    }

    pub fn create_ascii_snapshot(&self) -> anyhow::Result<ASCIISnapshot> {
        ASCIISnapshot::from_config(self.clone())
    }
}
