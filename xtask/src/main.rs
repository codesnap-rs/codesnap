use clap::{Parser, Subcommand};
use codesnap::config::SnapshotConfig;
use schemars::schema_for;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate JSON Schema
    Schema,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Schema => generate_schema(),
    }
}

fn generate_schema() {
    let schema = schema_for!(SnapshotConfig);
    let schema_json = serde_json::to_string_pretty(&schema).unwrap();
    let out_dir = Path::new("schemas");

    fs::create_dir_all(out_dir).unwrap();
    fs::write(out_dir.join("config.schema.json"), schema_json).unwrap();

    println!("✅ Schema generated at schemas/config.schema.json");
}
