use codesnap::config::{Code, CodeBuilder, Watermark, WatermarkBuilder, Window};

use crate::CLI;

pub fn create_watermark(cli: &CLI) -> anyhow::Result<Option<Watermark>> {
    if let Some(ref watermark) = cli.watermark {
        let mut watermark_builder = WatermarkBuilder::default();
        let watermark = watermark_builder
            .color(&cli.watermark_color)
            .content(watermark)
            .font_family(&cli.watermark_font_family)
            .build()?;

        return Ok(Some(watermark));
    }

    Ok(None)
}
