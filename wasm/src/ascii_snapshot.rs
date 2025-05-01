use codesnap::snapshot::ascii_snapshot::ASCIISnapshot as CodeSnapASCIISnapshot;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct ASCIISnapshot {
    snapshot: CodeSnapASCIISnapshot,
}

impl ASCIISnapshot {
    pub fn new(snapshot: CodeSnapASCIISnapshot) -> Self {
        Self { snapshot }
    }
}

#[wasm_bindgen]
impl ASCIISnapshot {
    pub fn raw_data(&self) -> String {
        match self.snapshot.raw_data().unwrap() {
            codesnap::snapshot::snapshot_data::SnapshotData::Text(data) => data,
            _ => panic!("Invalid conversion from SnapshotData to String"),
        }
    }
}
