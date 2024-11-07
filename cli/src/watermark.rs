use codesnap::config::{Watermark, WatermarkBuilder};

use crate::CLI;

pub fn create_watermark(
    cli: &CLI,
    config_watermark: Option<Watermark>,
) -> anyhow::Result<Option<Watermark>> {
    if cli.watermark.is_none() && config_watermark.is_none() {
        return Ok(None);
    }

    let watermark = if let Some(ref watermark) = cli.watermark {
        let watermark = WatermarkBuilder::default()
            .color(&cli.watermark_color)
            .content(watermark)
            .font_family(&cli.watermark_font_family)
            .build()?;

        Some(watermark)
    } else {
        None
    };

    Ok(config_watermark.or(watermark))
}
