use codesnap::config::{CodeBuilder, CodeSnap, Content};

pub fn main() -> anyhow::Result<()> {
    let code_content = Content::Code(
        CodeBuilder::default()
            .content(r#"print "Hello, World!""#)
            .language("python")
            .build()?,
    );

    let snapshot = CodeSnap::from_default_theme()?
        .content(code_content)
        .build()?
        .create_snapshot()?;

    // Copy the snapshot data to the clipboard
    snapshot.raw_data()?.copy()
}
