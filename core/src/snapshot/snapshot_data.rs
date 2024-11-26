use arboard::ImageData;
use tiny_skia::Pixmap;

use crate::utils::{clipboard::Clipboard, path::parse_file_name};
use std::fs::write;

pub enum SnapshotData {
    Image {
        data: Vec<u8>,
        width: usize,
        height: usize,
    },
    Text(String),
}

impl SnapshotData {
    pub fn from_pixmap(pixmap: &Pixmap, is_png_format: bool) -> Result<Self, anyhow::Error> {
        Ok(SnapshotData::Image {
            width: pixmap.width() as usize,
            height: pixmap.height() as usize,
            data: if is_png_format {
                pixmap.encode_png()?
            } else {
                pixmap.data().to_vec()
            },
        })
    }

    pub fn save(&self, save_path: &str) -> anyhow::Result<()> {
        let path = parse_file_name(save_path)?;

        Ok(match self {
            SnapshotData::Text(data) => write(path, data)?,
            SnapshotData::Image {
                data,
                width: _,
                height: _,
            } => write(path, data)?,
        })
    }

    pub fn copy(&self) -> anyhow::Result<()> {
        let mut clipboard = Clipboard::new()?;

        match self {
            SnapshotData::Text(data) => clipboard.set_text(data)?,
            SnapshotData::Image {
                data,
                width,
                height,
            } => {
                let image_data = ImageData {
                    width: *width,
                    height: *height,
                    bytes: data.into(),
                };

                clipboard.set_image(image_data)?;
            }
        };

        Ok(())
    }
}
