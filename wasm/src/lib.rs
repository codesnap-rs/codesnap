extern crate console_error_panic_hook;

mod ascii_snapshot;
mod image_snapshot;

use ascii_snapshot::ASCIISnapshot;
use codesnap::config::{CodeBuilder, CodeSnap, Content, SnapshotConfig};
use image_snapshot::ImageSnapshot;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

pub fn create_snapshot(code: &str, language: String) -> SnapshotConfig {
    let code_content = Content::Code(
        CodeBuilder::default()
            .content(code)
            .language(language)
            .build()
            .unwrap(),
    );

    CodeSnap::from_default_theme()
        .unwrap()
        .content(code_content)
        .build()
        .unwrap()
}

#[wasm_bindgen]
pub struct Snapshot {
    config: SnapshotConfig,
}

#[wasm_bindgen]
impl Snapshot {
    fn new(config: SnapshotConfig) -> Self {
        Self { config }
    }

    pub fn create_image_snapshot(&self) -> ImageSnapshot {
        ImageSnapshot::new(self.config.create_snapshot().unwrap())
    }

    pub fn create_ascii_snapshot(&self) -> ascii_snapshot::ASCIISnapshot {
        ASCIISnapshot::new(self.config.create_ascii_snapshot().unwrap())
    }
}

#[wasm_bindgen]
pub fn take_snapshot(code: &str, language: String) -> Snapshot {
    console_error_panic_hook::set_once();

    let config = create_snapshot(code, language);

    Snapshot::new(config)
}
