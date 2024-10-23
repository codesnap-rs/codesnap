use anyhow::bail;
use arboard::{Clipboard, ImageData};
use tiny_skia::Pixmap;

use crate::{
    utils::blur::{apply, ImageRefMut},
    utils::path::parse_save_path,
};

pub struct SnapShot {
    template: Pixmap,
}

impl SnapShot {
    pub fn new(template: Pixmap) -> Self {
        SnapShot { template }
    }

    pub fn copy(&self) {
        let premultplied_colors = self.template.pixels();
        let mut colors = premultplied_colors
            .iter()
            .map(|premultplied_color| {
                vec![
                    premultplied_color.red(),
                    premultplied_color.green(),
                    premultplied_color.blue(),
                    premultplied_color.alpha(),
                ]
            })
            .flatten()
            .collect::<Vec<u8>>();

        // let b = colors.as_rgba_mut();
        //
        // let ab = ImageRefMut::new(
        //     self.template.width() as u32,
        //     self.template.height() as u32,
        //     b,
        // );
        //
        // apply(10., 20., ab);
        //
        // Pixmap::from_bytes(
        //     self.template.width(),
        //     self.template.height(),
        //     self.template.color_type(),
        //     &colors,
        // );

        let img_data = ImageData {
            width: self.template.width() as usize,
            height: self.template.height() as usize,
            bytes: colors.clone().into(),
        };

        #[cfg(target_os = "linux")]
        std::thread::spawn(move || {
            Clipboard::new()
                .unwrap()
                .set()
                .wait()
                .image(img_data)
                .unwrap();
        });

        #[cfg(not(target_os = "linux"))]
        Clipboard::new().unwrap().set_image(img_data).unwrap();
    }

    pub fn save(&self, save_path: &str) -> anyhow::Result<()> {
        if !save_path.ends_with(".png") {
            bail!("The save_path must ends with .png");
        }

        let path = parse_save_path(save_path.to_string())?;

        self.template.save_png(path)?;

        Ok(())
    }

    pub fn draw_ascii(&self) {}
}
