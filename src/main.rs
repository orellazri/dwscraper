mod cli;
mod document;
mod issues;

use std::{env::current_dir, fs, path::Path, process::exit};

use clap::Parser;
use cli::Command;
use document::fetch_document;
use issues::{download_issue, find_last_issue_number};
use log::{error, info, LevelFilter};
use select::document::Document;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::cli::Cli;

fn download_range(document: &Document, issues: String, delim: usize, output_dir: &Path) {
    let last_issue_number = if let Some(number) = find_last_issue_number(document) {
        number
    } else {
        error!("failed to find last issue number");
        exit(1);
    };

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
        if let Err(e) = download_issue(issue_number, output_dir) {
            error!("failed to download issue {}", issue_number);
            eprintln!("{}", e);
        }
    }
}

fn download_single(issues: String, output_dir: &Path) {
    let issue_number: i32 = if let Ok(number) = issues.parse() {
        number
    } else {
        error!("failed to parse issue number");
        exit(1);
    };

    if let Err(e) = download_issue(issue_number, output_dir) {
        error!("failed to download issue {}", issue_number);
        eprintln!("{}", e);
    }
}

fn archive_issues(document: &Document, output_dir: &Path) {
    let last_issue_number = if let Some(number) = find_last_issue_number(document) {
        number
    } else {
        error!("failed to find last issue number");
        exit(1);
    };

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
            if let Err(e) = download_issue(issue_number, output_dir) {
                error!("failed to download issue {}", issue_number);
                eprintln!("{}", e);
            }
        }
    }
}

fn main() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let cli = Cli::parse();

    let output_dir = if let Some(dir) = cli.output {
        dir
    } else {
        current_dir().unwrap()
    };
    if !output_dir.is_dir() {
        error!("output directory does not exist");
        exit(1);
    }

    let document = if let Ok(doc) = fetch_document() {
        doc
    } else {
        error!("failed to fetch website content");
        exit(1);
    };

    match cli.command {
        Command::Download { issues } => {
            if let Some(delim) = issues.find(':') {
                download_range(&document, issues, delim, &output_dir);
            } else {
                download_single(issues, &output_dir);
            }
        }
        Command::Archive => {
            archive_issues(&document, &output_dir);
        }
    }
}
