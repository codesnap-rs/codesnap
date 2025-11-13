use thiserror::Error;

pub type Result<T> = std::result::Result<T, RenderError>;

#[derive(Debug, Error, Clone)]
pub enum RenderError {
    #[error("Highlight code failed!")]
    HighlightThemeLoadFailed,

    #[error("No such file {0}")]
    NoSuchFile(String),

    #[error("Failed to decode image data")]
    FailedDecodeImageData,
}
