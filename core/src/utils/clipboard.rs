use arboard::ImageData;
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
#[cfg(target_os = "linux")]
use std::time::{Duration, Instant};

#[cfg(target_os = "linux")]
const LINUX_CLIPBOARD_WAIT_DURATION: Duration = Duration::from_secs(1);

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
        self.aboard_clipboard
            .set()
            .wait_until(Instant::now() + LINUX_CLIPBOARD_WAIT_DURATION)
            .image(image_data)?;

        #[cfg(not(target_os = "linux"))]
        self.aboard_clipboard.set_image(image_data)?;

        Ok(())
    }

    pub fn set_text(&mut self, text: &str) -> Result<()> {
        #[cfg(target_os = "linux")]
        self.aboard_clipboard
            .set()
            .wait_until(Instant::now() + LINUX_CLIPBOARD_WAIT_DURATION)
            .text(text)?;

        #[cfg(not(target_os = "linux"))]
        self.aboard_clipboard.set_text(text)?;

        Ok(())
    }

    pub fn read(&mut self) -> Result<String> {
        self.aboard_clipboard.get_text()
    }
}
