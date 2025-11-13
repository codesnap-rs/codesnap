use codesnap::config::{CodeSnap, Content};

pub fn main() -> anyhow::Result<()> {
    let data = std::fs::read("core/examples/screenshot.png").unwrap();
    let code_content = Content::Image(data);

    let snapshot = CodeSnap::from_default_theme()?
        .content(code_content)
        .build()?
        .create_snapshot()?;

    // Copy the snapshot data to the clipboard
    snapshot.raw_data()?.copy()
}
