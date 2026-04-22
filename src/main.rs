use anyhow::{Context, Result};
use clap::Parser;
use labscope::cli::{Cli, Command};
use labscope::report;
use labscope::store::{self, Filter};
use std::fs;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Ingest { input, json } => {
            let events = store::load_events(&input)?;
            let summary = store::summarize(&events);

            if json {
                println!("{}", serde_json::to_string_pretty(&summary)?);
            } else {
                println!("validated {} events from {}", events.len(), input.display());
                print!("{}", report::render_summary_text(&summary));
            }
        }
        Command::Summary {
            input,
            filter,
            json,
        } => {
            let events = store::load_events(&input)?;
            let filtered = store::filter_events(&events, &Filter::from(&filter));
            let summary = store::summarize(&filtered);

            if json {
                println!("{}", serde_json::to_string_pretty(&summary)?);
            } else {
                print!("{}", report::render_summary_text(&summary));
            }
        }
        Command::Tail {
            input,
            lines,
            filter,
            json,
        } => {
            let events = store::load_events(&input)?;
            let filtered = store::filter_events(&events, &Filter::from(&filter));
            let tail_start = filtered.len().saturating_sub(lines);
            let tail = &filtered[tail_start..];

            if json {
                println!("{}", serde_json::to_string_pretty(&tail)?);
            } else {
                print!("{}", report::render_tail_text(tail));
            }
        }
        Command::ExportHtml {
            input,
            output,
            filter,
        } => {
            let events = store::load_events(&input)?;
            let filtered = store::filter_events(&events, &Filter::from(&filter));
            let summary = store::summarize(&filtered);
            let html = report::render_html(&summary, &filtered);

            fs::write(&output, html)
                .with_context(|| format!("failed to write report: {}", output.display()))?;

            println!(
                "wrote report to {} ({} events)",
                output.display(),
                filtered.len()
            );
        }
    }

    Ok(())
}
