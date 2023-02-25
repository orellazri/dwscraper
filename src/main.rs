mod cli;
mod document;
mod issues;

use clap::Parser;
use cli::Command;
use issues::download_issue;

use crate::{cli::Cli, document::fetch_document, issues::find_last_issue_number};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Download { issue } => {
            download_issue(issue).expect("failed to download issue");
        }
    }

    // let document = fetch_document();
    // let last_issue_number =
    //     find_last_issue_number(&document).expect("failed to find last issue link");
    // download_issue(last_issue_number).expect("failed to download issue");
}
