mod cli;
mod document;
mod issues;

use std::{env::current_dir, process::exit};

use clap::Parser;
use cli::Command;
use document::fetch_document;
use issues::{download_issue, find_last_issue_number};
use log::{error, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::cli::Cli;

fn main() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let cli = Cli::parse();

    match cli.command {
        Command::Download { issues } => {
            let output_dir = if let Some(dir) = cli.output {
                dir
            } else {
                current_dir().unwrap()
            };

            if !output_dir.is_dir() {
                error!("output directory does not exist");
                exit(1);
            }

            if let Some(delim) = issues.find(':') {
                let document = fetch_document();
                let last_issue_number =
                    find_last_issue_number(&document).expect("failed to find last issue number");

                let start: i32 = issues[..delim].parse().unwrap_or(1);
                let end: i32 = issues[delim + 1..].parse().unwrap_or(last_issue_number);

                if start < 1 || end > last_issue_number {
                    error!(
                        "invalid issue range, should be between 1-{}",
                        last_issue_number
                    );
                    exit(1);
                }

                for issue_number in start..=end {
                    download_issue(issue_number, &output_dir).expect("failed to download issue");
                }
            } else {
                let issue_number: i32 = issues.parse().expect("failed to parse issue number");
                download_issue(issue_number, &output_dir).expect("failed to download issue");
            }
        }
    }
}
