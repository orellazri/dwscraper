mod document;
mod issues;

use issues::download_issue;

use crate::{document::fetch_document, issues::find_last_issue_number};

fn main() {
    let document = fetch_document();
    let last_issue_number =
        find_last_issue_number(&document).expect("failed to find last issue link");
    download_issue(last_issue_number).expect("failed to download issue");
}
