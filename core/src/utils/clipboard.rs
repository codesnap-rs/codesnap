use arboard::ImageData;
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
#[cfg(target_os = "linux")]
use std::thread;

pub struct Clipboard {
    aboard_clipboard: arboard::Clipboard,
}

pub type Result<T> = std::result::Result<T, arboard::Error>;

impl Clipboard {
    pub fn new() -> Result<Self> {
        Ok(Self {
            aboard_clipboard: arboard::Clipboard::new()?,
        })
    }

    pub fn set_image(&mut self, image_data: ImageData) -> Result<()> {
        #[cfg(target_os = "linux")]
        thread::scope(|s| -> Result<()> {
            s.spawn(|| -> Result<()> { self.aboard_clipboard.set().wait().image(image_data) })
                .join()
                .unwrap()
        })?;

        #[cfg(not(target_os = "linux"))]
        self.aboard_clipboard.set_image(image_data)?;

        Ok(())
    }

    pub fn set_text(&mut self, text: &str) -> Result<()> {
        #[cfg(target_os = "linux")]
        thread::scope(|s| -> Result<()> {
            s.spawn(move || -> Result<()> { self.aboard_clipboard.set().wait().text(text) })
                .join()
                .unwrap()
        })?;

        #[cfg(not(target_os = "linux"))]
        self.aboard_clipboard.set_text(text)?;

        Ok(())
    }

    pub fn read(&mut self) -> Result<String> {
        self.aboard_clipboard.get_text()
    }
}
