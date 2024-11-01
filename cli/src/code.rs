use std::fs::{metadata, read_to_string};

use anyhow::bail;
use codesnap::config::{Breadcrumbs, Code, CodeBuilder, HighlightLine};

use crate::CLI;

pub fn create_code(cli: &CLI) -> anyhow::Result<Code> {
    let code_snippet = get_code_snippet(cli)?;
    let mut code_builder = CodeBuilder::default();
    let mut code = code_builder
        .content(&code_snippet)
        .theme(&cli.code_theme)
        .font_family(&cli.code_font_family)
        .build()?;

    code.file_path = cli.file.clone();
    code.language = cli.language.clone();
    code.breadcrumbs = create_breadcrumbs(&cli);
    code.highlight_lines = create_highlight_lines(&cli)?;

    Ok(code)
}

fn create_highlight_lines(cli: &CLI) -> Result<Vec<HighlightLine>, serde_json::Error> {
    match cli.highlight_lines {
        Some(ref highlight_lines) => serde_json::from_str::<Vec<HighlightLine>>(highlight_lines),
        None => Ok(vec![]),
    }
}

fn create_breadcrumbs(cli: &CLI) -> Option<Breadcrumbs> {
    if cli.has_breadcrumbs {
        return Some(Breadcrumbs {
            separator: cli.breadcrumbs_separator.clone(),
            font_family: cli.breadcrumbs_font_family.clone(),
            color: cli
                .breadcrumbs_color
                .clone()
                .unwrap_or(String::from("#80848b")),
        });
    }

    None
}

fn get_code_snippet(cli: &CLI) -> anyhow::Result<String> {
    if cli.file.is_some() && cli.code.is_some() {
        bail!("You can only specify one of the file or code option");
    }

    if cli.file.is_none() && cli.code.is_none() {
        bail!("You must specify one of the file or code option");
    }

    match cli.file {
        Some(ref file_path) => {
            if !metadata(&file_path)?.is_file() {
                bail!("The file path is not a file");
            }

            Ok(read_to_string(file_path)?)
        }
        None => Ok(cli.code.clone().unwrap().clone()),
    }
}
