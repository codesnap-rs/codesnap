pub trait Snapshot {
    fn copy(&self) -> anyhow::Result<()>;

    fn save(&self, save_path: &str) -> anyhow::Result<()>;
}
