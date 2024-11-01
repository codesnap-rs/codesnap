use codesnap::config::{Border, Code, CodeBuilder, Margin, TitleConfig, Window, WindowBuilder};

use crate::CLI;

pub fn create_window(cli: &CLI) -> anyhow::Result<Window> {
    let mut window_builder = WindowBuilder::default();
    let mut window = window_builder
        .mac_window_bar(cli.mac_window_bar)
        .shadow(cli.shadow)
        .margin(Margin {
            x: cli.margin_x,
            y: cli.margin_y,
        })
        .build()?;

    window.title = create_title(cli);
    window.border = create_border(cli);

    Ok(window)
}

fn create_border(cli: &CLI) -> Option<Border> {
    cli.has_border.then(|| Border {
        color: cli.border_color.clone(),
    })
}

fn create_title(cli: &CLI) -> Option<TitleConfig> {
    cli.title.as_ref().and_then(|title| {
        Some(TitleConfig {
            title: title.clone(),
            font_family: cli.title_font_family.clone(),
            color: cli.title_color.clone(),
        })
    })
}
