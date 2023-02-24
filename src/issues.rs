use std::{error::Error, fs::File, io};

use select::{document::Document, predicate::Name};

use crate::document::SITE_URL;

fn get_issue_number_from_link(issue_link: &str) -> Option<i32> {
    if let Ok(number) = issue_link[issue_link.find("issue").unwrap() + "issue".len()..].parse() {
        return Some(number);
    }

    None
}

pub fn find_last_issue_number(document: &Document) -> Option<i32> {
    let issue_link = document
        .find(Name("a"))
        .find(|e| e.text() == "להורדת הגליון האחרון")?
        .attr("href")?;

    let issue_number: i32 = get_issue_number_from_link(issue_link)?;

    Some(issue_number)
}

pub fn download_issue(issue_number: i32) -> Result<(), Box<dyn Error>> {
    let issue_hex = format!("{:#04x}", issue_number);
    let issue_url = format!(
        "{}/files/Zines/{}/DigitalWhisper{}.pdf",
        SITE_URL, issue_hex, issue_number
    );

    let mut response = reqwest::blocking::get(issue_url)?;
    let mut output = File::create(format!("{}.pdf", issue_number))?;
    io::copy(&mut response, &mut output)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::document;

    use super::*;

    #[test]
    fn can_find_last_issue_number() {
        let document = document::fetch_document();
        let last_issue_number = find_last_issue_number(&document);
        assert!(last_issue_number.is_some());
    }

    #[test]
    fn can_get_valid_issue_number_from_link() {
        let issue_url = format!("{}/issue1", SITE_URL);

        let issue_number = get_issue_number_from_link(&issue_url);
        assert!(issue_number.is_some());
        assert!(issue_number.unwrap() == 1);
    }

    #[test]
    fn can_get_invalid_issue_number_from_link() {
        let issue_url = format!("{}/issue", SITE_URL);
        let issue_number = get_issue_number_from_link(&issue_url);
        assert!(issue_number.is_none());

        let issue_url = format!("{}/issuenothing", SITE_URL);
        let issue_number = get_issue_number_from_link(&issue_url);
        assert!(issue_number.is_none());

        let issue_url = format!("{}/issue1a", SITE_URL);
        let issue_number = get_issue_number_from_link(&issue_url);
        assert!(issue_number.is_none());
    }

    // #[test]
    // fn can_download_issue() {
    //     download_issue(1);
    // }
}
