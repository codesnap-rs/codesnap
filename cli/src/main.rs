mod code;
mod logger;
mod watermark;
mod window;

use std::fs::read_to_string;

use clap::Parser;
use clap::Subcommand;
use code::create_code;
use codesnap::config::CodeSnap;
use codesnap::config::SnapshotConfig;
use codesnap::snapshot::snapshot::Snapshot;
use watermark::create_watermark;
use window::create_window;

/// CodeSnap is a CLI tool to generate beautiful snapshots of your code from terminal.
#[derive(Parser)]
#[command(bin_name = "codesnap")]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct CLI {
    /// Path to the file to snapshot
    #[arg(short, long)]
    file: Option<String>,

    /// Code snippet to snapshot
    #[arg(short, long)]
    code: Option<String>,

    /// Output path for the snapshot, currently CodeSnap supports SVG format and PNG format
    /// If output is directory, CodeSnap will generate a temporary file name to save the snapshot
    /// to the directory.
    #[arg(short, long)]
    output: Option<String>,

    /// Copy the snapshot to clipboard
    #[arg(long)]
    to_clipboard: bool,

    /// Font family for the code snippet
    #[arg(long)]
    code_font_family: Option<String>,

    /// Code theme for the code snippet
    #[arg(long)]
    code_theme: Option<String>,

    /// Breadcrumbs is a useful and unique feature in CodeSnap, it shows the path of the file
    /// so that users can know where the code snippet comes from.
    #[arg(long)]
    has_breadcrumbs: bool,

    /// Breadcrumbs separator is the character to separate the path in breadcrumbs
    /// Default is `/`
    #[arg(long)]
    breadcrumbs_separator: Option<String>,

    /// Breadcrumbs font family
    #[arg(long)]
    breadcrumbs_font_family: Option<String>,

    /// Breadcrumbs font color
    #[arg(long)]
    breadcrumbs_color: Option<String>,

    /// Set start line number to display line numbers
    #[arg(long)]
    start_line_number: Option<u32>,

    /// Line number font color
    #[arg(long, default_value = "#495162")]
    line_number_color: String,

    /// CodeSnap allows users to highlight multiple lines with custom highlight color in the code snippet
    /// The content of highlight_lines is JSON string, for example highlight the first line and the
    /// 3rd to 5th lines:
    /// "[
    ///   [1, "#ff0000"],
    ///   [3, 5, "#00ff00"]
    /// ]"
    ///
    #[arg(long)]
    highlight_lines: Option<String>,

    /// Set the language of the code snippet, If you using the `file` option, CodeSnap will
    /// automatically detect the language from the file extension.
    #[arg(long, short)]
    language: Option<String>,

    /// Set watermark for the code snippet
    #[arg(long, short)]
    watermark: Option<String>,

    /// Watermark font family
    #[arg(long, default_value = "Pacifico")]
    watermark_font_family: String,

    /// Watermark font color
    #[arg(long, default_value = "#ffffff")]
    watermark_color: String,

    /// Set window shadow radius
    #[arg(long)]
    shadow: Option<f32>,

    /// Display MacOS style window bar
    #[arg(long)]
    mac_window_bar: Option<bool>,

    /// Display window border
    #[arg(long, default_value_t = true)]
    has_border: bool,

    /// Window border color
    #[arg(long, default_value = "#ffffff30")]
    border_color: String,

    /// Set horizontal margin of window
    #[arg(long)]
    margin_x: Option<f32>,

    /// Set vertical margin of window
    #[arg(long)]
    margin_y: Option<f32>,

    /// Set title of the window
    #[arg(long)]
    title: Option<String>,

    /// CodeSnap supports scaling the snapshot, the default scale factor is 3 for better quality
    #[arg(long, default_value_t = 3)]
    scale_factor: u8,

    /// Title font family
    #[arg(long, default_value = "")]
    title_font_family: String,

    /// Title font color
    #[arg(long, default_value = "#aca9b2")]
    title_color: String,

    /// CodeSnap supports custom themes, you can set the folder path of the themes
    #[arg(long)]
    themes_folder: Option<String>,

    /// CodeSnap supports custom fonts, you can set the folder path of the fonts, or CodeSnap will
    /// use the system fonts.
    #[arg(long)]
    fonts_folder: Option<String>,

    /// Set background color of the snapshot
    #[arg(long)]
    background: Option<String>,

    /// To generate ASCII snapshot ranther than image snapshot
    #[arg(long)]
    ascii: bool,

    #[arg(long)]
    config: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Save(Args),
    Copy(Args),
}

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    output: String,
}

fn generate_snapshot() -> anyhow::Result<()> {
    let cli = CLI::parse();

    let mut codesnap_default = if let Some(ref config) = cli.config {
        let content = read_to_string(config)?;

        CodeSnap::from_config(&content)?
    } else {
        CodeSnap::default()
    };

    let mut codesnap = codesnap_default
        .map_code(|code| create_code(&cli, code))?
        .map_watermark(|watermark| create_watermark(&cli, watermark))?
        .map_window(|window| create_window(&cli, window))?
        .scale_factor(cli.scale_factor)
        .build()?;

    codesnap.themes_folder = cli.themes_folder.clone().or(codesnap.themes_folder);
    codesnap.fonts_folder = cli.fonts_folder.clone().or(codesnap.fonts_folder);

    if cli.ascii {
        execute(&cli, codesnap.create_ascii_snapshot()?, codesnap)?;
    } else {
        execute(&cli, codesnap.create_snapshot()?, codesnap)?;
    }

    Ok(())
}

// Execute copy or save action to consuming snapshot
fn execute(cli: &CLI, snapshot: impl Snapshot, codesnap: SnapshotConfig) -> anyhow::Result<()> {
    if cli.to_clipboard {
        return snapshot.copy();
    }

    if let Some(ref output) = cli.output {
        if output.ends_with(".svg") {
            return codesnap.create_svg_snapshot()?.save(&output);
        }

        snapshot.save(&output)?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = generate_snapshot() {
        logger::error(&err.to_string());
    }
}
