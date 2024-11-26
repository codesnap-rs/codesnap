use std::{
    fs::{metadata, read_to_string},
    io::{stdin, BufReader, IsTerminal, Read},
    process,
};

use anyhow::bail;
use clap::CommandFactory;
use codesnap::config::{Breadcrumbs, Code, CodeBuilder, HighlightLine};

use crate::{CLI, STDIN_CODE_DEFAULT_CHAR};

pub fn create_code(cli: &CLI, config_code: Code) -> anyhow::Result<Code> {
    let code_snippet = get_code_snippet(cli)?;
    let mut code_builder = CodeBuilder::from_code(config_code.clone());
    let mut code = code_builder.content(&code_snippet).build()?;

    code.theme = cli.code_theme.clone().unwrap_or(config_code.theme);
    code.font_family = cli
        .code_font_family
        .clone()
        .unwrap_or(config_code.font_family);
    code.file_path = cli.from_file.clone().or(config_code.file_path);
    code.language = cli.language.clone().or(config_code.language);
    code.breadcrumbs = create_breadcrumbs(&cli).or(config_code.breadcrumbs);
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
    if cli.from_file.is_some() && cli.from_code.is_some() {
        bail!("You can only specify one of the file or code option");
    }

    if cli.from_file.is_none() && cli.from_code.is_none() {
        bail!("You must specify one of the file or code option");
    }

    match cli.from_file {
        Some(ref file_path) => {
            if !metadata(&file_path)?.is_file() {
                bail!("The file path is not a file");
            }

            Ok(read_to_string(file_path)?)
        }
        None => {
            let code = cli.from_code.clone().unwrap();

            // Read code from pipe if the code option is "-"
            if code == STDIN_CODE_DEFAULT_CHAR {
                // If input come from terminal, print help and exit
                if stdin().is_terminal() {
                    CLI::command().print_help()?;
                    process::exit(2);
                }

                let mut content = String::new();

                BufReader::new(stdin().lock()).read_to_string(&mut content)?;

                return Ok(content);
            }

            Ok(code)
        }
    }
}
