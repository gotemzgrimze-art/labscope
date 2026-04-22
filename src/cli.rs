use crate::model::Severity;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "labscope")]
#[command(version)]
#[command(about = "Passive device telemetry and recon workbench")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Args, Default)]
pub struct FilterArgs {
    /// Match a specific device id
    #[arg(long)]
    pub device: Option<String>,

    /// Match a specific event name
    #[arg(long)]
    pub event: Option<String>,

    /// Match a severity level
    #[arg(long)]
    pub severity: Option<Severity>,

    /// Match a Wi-Fi channel
    #[arg(long)]
    pub channel: Option<u16>,

    /// Match a tag
    #[arg(long)]
    pub tag: Option<String>,

    /// Case-insensitive substring search across message, event, device, tags, and metadata
    #[arg(long)]
    pub contains: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Validate and inventory events from a JSONL file or stdin (`-`)
    Ingest {
        input: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// Print a high-level session summary, optionally filtered
    Summary {
        input: PathBuf,
        #[command(flatten)]
        filter: FilterArgs,
        #[arg(long)]
        json: bool,
    },
    /// Print the most recent matching events
    Tail {
        input: PathBuf,
        #[arg(long, default_value_t = 10)]
        lines: usize,
        #[command(flatten)]
        filter: FilterArgs,
        #[arg(long)]
        json: bool,
    },
    /// Render a static HTML report
    ExportHtml {
        input: PathBuf,
        output: PathBuf,
        #[command(flatten)]
        filter: FilterArgs,
    },
}
