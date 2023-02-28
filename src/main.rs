mod cli;
mod document;
mod issues;

use std::{env::current_dir, fs, path::Path, process::exit};

use clap::Parser;
use cli::Command;
use document::fetch_document;
use issues::{download_issue, find_last_issue_number};
use log::{error, info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::cli::Cli;

fn download_range(issues: String, delim: usize, output_dir: &Path, last_issue_number: i32) {
    let start: i32 = issues[..delim].parse().unwrap_or(1);
    let end: i32 = issues[delim + 1..].parse().unwrap_or(last_issue_number);

    if start < 1 || end > last_issue_number {
        error!("invalid issue range, should be between 1-{}", last_issue_number);
        exit(1);
    }

    for issue_number in start..=end {
        download_issue(issue_number, output_dir).unwrap_or_else(|err| {
            error!("failed to download issue {}", issue_number);
            eprintln!("{}", err);
        });
    }
}

fn download_single(issues: String, output_dir: &Path, last_issue_number: i32) {
    let issue_number: i32 = if let Ok(number) = issues.parse() {
        number
    } else if issues == "last" {
        last_issue_number
    } else {
        error!("invalid issue number");
        exit(1);
    };

    download_issue(issue_number, output_dir).unwrap_or_else(|err| {
        error!("failed to download issue {}", issue_number);
        eprintln!("{}", err);
    });
}

fn archive_issues(output_dir: &Path, last_issue_number: i32) {
    // Add existing issues from disk
    let mut existing_issues: Vec<i32> = Vec::with_capacity(last_issue_number as usize);
    for file in fs::read_dir(output_dir).unwrap() {
        let path = file.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "pdf" {
                if let Some(stem) = path.file_stem() {
                    if let Ok(issue_number) = stem.to_str().unwrap().parse::<i32>() {
                        existing_issues.push(issue_number);
                    }
                }
            }
        }
    }

    info!("found {} issues on disk", existing_issues.len());
    info!(
        "need to download {} remaining issues",
        last_issue_number - (existing_issues.len() as i32)
    );

    // Download missing issues
    for issue_number in 1..=last_issue_number {
        if !existing_issues.contains(&issue_number) {
            download_issue(issue_number, output_dir).unwrap_or_else(|err| {
                error!("failed to download issue {}", issue_number);
                eprintln!("{}", err);
            });
        }
    }
}

fn main() {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    let cli = Cli::parse();

    // Initialize pre-requisite variables needed for operation
    let output_dir = if let Some(dir) = cli.output {
        dir
    } else {
        current_dir().unwrap()
    };
    if !output_dir.is_dir() {
        error!("output directory does not exist");
        exit(1);
    }

    let document = fetch_document().unwrap_or_else(|_| {
        error!("failed to fetch website content");
        exit(1);
    });

    let last_issue_number = find_last_issue_number(&document).unwrap_or_else(|| {
        error!("failed to find last issue number");
        exit(1);
    });

    // Run relevant command
    match cli.command {
        Command::Download { issues } => {
            if let Some(delim) = issues.find(':') {
                download_range(issues, delim, &output_dir, last_issue_number);
            } else {
                download_single(issues, &output_dir, last_issue_number);
            }
        }
        Command::Archive => {
            archive_issues(&output_dir, last_issue_number);
        }
    }
}
