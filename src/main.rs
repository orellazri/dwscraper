mod document;
mod issues;

use crate::{
    document::fetch_document,
    issues::{download_issue, find_last_issue_link},
};

fn main() {
    let document = fetch_document();
    let last_issue_link = find_last_issue_link(&document).expect("failed to find last issue link");
    let issue_number: i32 = last_issue_link
        [last_issue_link.find("issue").unwrap() + "issue".len()..]
        .parse()
        .expect("failed to get issue number");
    download_issue(issue_number).expect("failed to download issue");
}
