use anyhow::Result;

use crate::config::SnapshotConfig;

pub trait Snapshot {
    fn copy(&self) -> Result<()>;
    fn save(&self, save_path: &str) -> Result<()>;
    fn from_config(config: SnapshotConfig) -> Result<Self>
    where
        Self: Sized;
}
