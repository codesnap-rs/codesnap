use codesnap::config::{Watermark, WatermarkBuilder};

use crate::CLI;

pub fn create_watermark(
    cli: &CLI,
    config_watermark: Option<Watermark>,
) -> anyhow::Result<Option<Watermark>> {
    if cli.watermark.is_none() && config_watermark.is_none() {
        return Ok(None);
    }

    if cli.disable_watermark == true {
        return Ok(None);
    }

    let watermark = cli
        .watermark
        .clone()
        .or(config_watermark
            .clone()
            .and_then(|config_watermark| Some(config_watermark.content)))
        .and_then(|content| {
            Some(
                WatermarkBuilder::default()
                    .color(
                        cli.watermark_color
                            .clone()
                            .or(config_watermark
                                .as_ref()
                                .and_then(|watermark| Some(watermark.color.clone())))
                            .unwrap_or(String::from("#ffffff")),
                    )
                    .content(content)
                    .font_family(
                        &cli.watermark_font_family
                            .clone()
                            .or(config_watermark
                                .and_then(|watermark| Some(watermark.font_family.clone())))
                            .unwrap_or(String::from("Pacifico")),
                    )
                    .build()
                    .unwrap(),
            )
        });

    Ok(watermark)
}
