use codesnap::config::{Breadcrumbs, CodeConfig, CodeConfigBuilder};

use crate::CLI;

pub fn create_code_config(cli: &CLI, code_config: CodeConfig) -> anyhow::Result<CodeConfig> {
    let mut parsed_code_config = CodeConfigBuilder::default().build()?;

    parsed_code_config.breadcrumbs = map_breadcrumbs(&cli, code_config.breadcrumbs);
    parsed_code_config.font_family = cli
        .code_font_family
        .clone()
        .unwrap_or(parsed_code_config.font_family);

    Ok(parsed_code_config)
}

fn map_breadcrumbs(cli: &CLI, breadcrumbs_config: Breadcrumbs) -> Breadcrumbs {
    Breadcrumbs {
        separator: cli
            .breadcrumbs_separator
            .clone()
            .unwrap_or(breadcrumbs_config.separator),
        font_family: cli
            .breadcrumbs_font_family
            .clone()
            .unwrap_or(breadcrumbs_config.font_family),
        color: cli
            .breadcrumbs_color
            .clone()
            .unwrap_or(breadcrumbs_config.color),
    }
}
