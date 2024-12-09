use std::{
    fs::{metadata, read_to_string},
    io::{stdin, BufReader, IsTerminal, Read},
    process,
};

use anyhow::bail;
use clap::CommandFactory;
use codesnap::{
    config::{Breadcrumbs, Code, CodeBuilder, HighlightLine, LineNumberBuilder},
    utils::clipboard::Clipboard,
};

use crate::{range::Range, CLI, STDIN_CODE_DEFAULT_CHAR};

pub fn create_code(cli: &CLI, config_code: Code) -> anyhow::Result<Code> {
    let range = Range::from_opt_string(cli.range.clone())?;
    let code_snippet = get_code_snippet(cli)?;
    let parsed_range = range.parse_range(&code_snippet)?;
    let parsed_code_snippet = parsed_range.cut_code_snippet(&code_snippet)?;
    let mut code_builder = CodeBuilder::from_code(config_code.clone());
    let mut code = code_builder.content(&parsed_code_snippet).build()?;

    code.line_number = cli.has_line_number.then(|| {
        LineNumberBuilder::default()
            .start_number(parsed_range.0 as u32)
            .color(cli.line_number_color.clone())
            .build()
            .unwrap()
    });
    code.theme = cli.code_theme.clone().unwrap_or(config_code.theme);
    code.font_family = cli
        .code_font_family
        .clone()
        .unwrap_or(config_code.font_family);
    code.file_path = cli.from_file.clone().or(config_code.file_path);
    code.language = cli.language.clone().or(config_code.language);
    code.breadcrumbs = create_breadcrumbs(&cli).or(config_code.breadcrumbs);
    code.highlight_lines = create_highlight_lines(&cli, &code_snippet)?;

    Ok(code)
}

fn create_highlight_lines(cli: &CLI, code_snippet: &str) -> anyhow::Result<Vec<HighlightLine>> {
    if let Some(ref raw_highlight_lines) = cli.raw_highlight_lines {
        let highlight_lines = serde_json::from_str::<Vec<HighlightLine>>(raw_highlight_lines)?;

        return Ok(highlight_lines);
    }

    let highlight_lines = match cli.highlight_range {
        Some(ref highlight_range) => {
            let Range(start, end) = Range::from_str(&highlight_range)?.parse_range(code_snippet)?;

            vec![HighlightLine::Range(
                start as u32,
                end as u32,
                cli.highlight_range_color.clone(),
            )]
        }
        None => vec![],
    };

    Ok(highlight_lines)
}

fn create_breadcrumbs(cli: &CLI) -> Option<Breadcrumbs> {
    cli.has_breadcrumbs.then(|| Breadcrumbs {
        separator: cli.breadcrumbs_separator.clone(),
        font_family: Some(cli.breadcrumbs_font_family.clone()),
        color: cli
            .breadcrumbs_color
            .clone()
            .unwrap_or(String::from("#80848b")),
    })
}

fn get_code_snippet(cli: &CLI) -> anyhow::Result<String> {
    if let Some(ref file_path) = cli.from_file {
        if !metadata(file_path)?.is_file() {
            bail!("The file path is not a file");
        }

        return Ok(read_to_string(file_path)?);
    }

    if let Some(ref code) = cli.from_code {
        // Read code from pipe if the code option is "-"
        return Ok(if code == STDIN_CODE_DEFAULT_CHAR {
            // If input come from terminal, print help and exit
            if stdin().is_terminal() {
                CLI::command().print_help()?;
                process::exit(2);
            }

            let mut content = String::new();

            BufReader::new(stdin().lock()).read_to_string(&mut content)?;

            content
        } else {
            code.clone()
        });
    }

    if cli.from_clipboard {
        let content = Clipboard::new()?.read()?;

        return Ok(content);
    }

    bail!("No code snippet provided");
}
