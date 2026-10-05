use codesnap::config::{Border, Margin, Shadow, TitleConfig, Window, WindowBuilder};

use crate::CLI;

pub fn create_window(cli: &CLI, config_window: Window) -> anyhow::Result<Window> {
    let mut window = WindowBuilder::from_window(config_window.clone()).build()?;

    window.margin = Margin {
        x: cli.margin_x.unwrap_or(config_window.margin.x),
        y: cli.margin_y.unwrap_or(config_window.margin.y),
    };
    window.shadow = Shadow {
        color: cli
            .shadow_color
            .clone()
            .unwrap_or(config_window.shadow.color),
        radius: cli.shadow_radius.unwrap_or(config_window.shadow.radius),
    };
    window.mac_window_bar = cli.mac_window_bar.unwrap_or(config_window.mac_window_bar);
    window.title_config = create_title(cli, config_window.title_config);
    window.border = create_border(cli, config_window.border);

    Ok(window)
}

fn create_border(cli: &CLI, config: Border) -> Border {
    Border {
        color: cli.border_color.clone().unwrap_or(config.color),
        width: match cli.has_border {
            Some(false) => 0.,
            Some(true) if config.width <= 0. => 1.,
            _ => config.width,
        },
    }
}

fn create_title(cli: &CLI, config: TitleConfig) -> TitleConfig {
    TitleConfig {
        font_family: cli.title_font_family.clone().unwrap_or(config.font_family),
        color: cli.title_color.clone().unwrap_or(config.color),
    }
}
