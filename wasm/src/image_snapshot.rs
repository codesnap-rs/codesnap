use codesnap::snapshot::{
    image_snapshot::ImageSnapshot as CodeSnapImageSnapshot, snapshot_data::SnapshotData,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct ImageData {
    pub width: usize,
    pub height: usize,
    data: Vec<u8>,
}

impl From<SnapshotData> for ImageData {
    fn from(value: SnapshotData) -> Self {
        match value {
            SnapshotData::Image {
                data,
                width,
                height,
            } => ImageData {
                data,
                width,
                height,
            },
            _ => panic!("Invalid conversion from SnapshotData to ImageData"),
        }
    }
}

#[wasm_bindgen]
impl ImageData {
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[wasm_bindgen]
pub struct ImageSnapshot {
    snapshot: CodeSnapImageSnapshot,
}

impl ImageSnapshot {
    pub fn new(snapshot: CodeSnapImageSnapshot) -> Self {
        Self { snapshot }
    }
}

#[wasm_bindgen]
impl ImageSnapshot {
    pub fn raw_data(&self) -> ImageData {
        self.snapshot.raw_data().unwrap().into()
    }

    pub fn png_data(&self) -> ImageData {
        self.snapshot.png_data().unwrap().into()
    }

    pub fn svg_data(&self) -> ImageData {
        self.snapshot.svg_data().unwrap().into()
    }
}
