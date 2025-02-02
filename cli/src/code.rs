use std::{
    fs::{metadata, read_to_string},
    io::{stdin, BufReader, IsTerminal, Read},
    process::{self, Command},
};

use anyhow::bail;
use clap::CommandFactory;
use codesnap::{
    config::{
        Code, CodeBuilder, CommandLineContent, CommandLineContentBuilder, Content, HighlightLine,
    },
    utils::clipboard::Clipboard,
};

use crate::{highlight::HighlightLineRange, range::Range, CLI, STDIN_CODE_DEFAULT_CHAR};

pub fn create_code(cli: &CLI, code_config: Code) -> anyhow::Result<Content> {
    let code = match cli.execute[..] {
        [] => {
            let range = Range::from_opt_string(cli.range.clone())?;
            let code_snippet = get_code_snippet(cli)?;
            let parsed_range = range.parse_range(&code_snippet)?;
            let parsed_code_snippet = parsed_range.cut_code_snippet(&code_snippet)?;
            let mut code = CodeBuilder::default()
                .content(parsed_code_snippet)
                .start_line_number(parsed_range.0 as u32)
                .build()?;

            code.file_path = cli.from_file.clone().or(code_config.file_path);
            code.language = cli.language.clone().or(code_config.language);
            code.highlight_lines = create_highlight_lines(&cli, parsed_range, &code_snippet)?;

            Content::Code(code)
        }
        _ => {
            let command_content = cli
                .execute
                .clone()
                .into_iter()
                .map(|command| {
                    let output = execute_command(&command);

                    CommandLineContentBuilder::default()
                        .full_command(&command)
                        .content(output)
                        .build()
                        .unwrap()
                })
                .collect::<Vec<CommandLineContent>>();

            Content::CommandOutput(command_content)
        }
    };

    Ok(code)
}

fn execute_command(command: &str) -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    };
    let msg = if !output.status.success() {
        output.stderr
    } else {
        output.stdout
    };

    String::from_utf8_lossy(&msg).into_owned()
}

fn create_highlight_lines(
    cli: &CLI,
    code_snippet_range: Range<usize>,
    code_snippet: &str,
) -> anyhow::Result<Vec<HighlightLine>> {
    if let Some(ref raw_highlight_lines) = cli.raw_highlight_lines {
        let highlight_lines = serde_json::from_str::<Vec<HighlightLine>>(raw_highlight_lines)?;

        return Ok(highlight_lines);
    }

    let highlight_range = HighlightLineRange::from(
        code_snippet_range,
        code_snippet,
        cli.relative_highlight_range,
    )?;
    let highlight_lines = cli
        .highlight_range
        .clone()
        .and_then(|range| {
            Some(highlight_range.create_highlight_lines(&range, &cli.highlight_range_color))
        })
        .unwrap_or(Ok(vec![]))?;
    let delete_highlight_lines = highlight_range
        .create_multiple_highlight_lines(&cli.delete_line, &cli.delete_line_color)?;
    let new_highlight_lines =
        highlight_range.create_multiple_highlight_lines(&cli.add_line, &cli.add_line_color)?;

    Ok([highlight_lines, delete_highlight_lines, new_highlight_lines].concat())
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
