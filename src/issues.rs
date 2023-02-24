use std::{error::Error, fs::File, io};

use select::{document::Document, predicate::Name};

use crate::document::SITE_URL;

pub fn find_last_issue_link(document: &Document) -> Option<&str> {
    let last_issue_link = document
        .find(Name("a"))
        .find(|e| e.text() == "להורדת הגליון האחרון")?
        .attr("href")?;

    Some(last_issue_link)
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
    fn can_find_last_issue_link() {
        let document = document::fetch_document();
        let link = find_last_issue_link(&document).unwrap();
        assert!(!link.is_empty());
    }
}
