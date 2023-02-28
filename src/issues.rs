use std::{error::Error, fs::File, io, path::Path};

use log::info;
use select::{document::Document, predicate::Name};

use crate::document::SITE_URL;

fn get_issue_number_from_link(issue_link: &str) -> Option<i32> {
    if let Some(index) = issue_link.find("issue") {
        return issue_link[index + "issue".len()..].parse().ok();
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

pub fn download_issue(issue_number: i32, output_dir: &Path) -> Result<(), Box<dyn Error>> {
    info!("downloading issue {issue_number}");

    let issue_hex = format!("{:#04X}", issue_number);
    let issue_url = format!("{SITE_URL}/files/Zines/{issue_hex}/DigitalWhisper{issue_number}.pdf",);

    let mut output_path = output_dir.to_path_buf();
    output_path.push(issue_number.to_string());
    output_path.set_extension("pdf");

    let mut response = reqwest::blocking::get(issue_url)?;
    let mut output = File::create(output_path)?;
    io::copy(&mut response, &mut output)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env::temp_dir, fs, os::unix::prelude::MetadataExt};

    use crate::document;

    use super::*;

    fn check_pdf_file_is_valid(file: &Path) -> bool {
        let contents = fs::read(file).unwrap();

        contents[0] == 0x25 && contents[1] == 0x50 && contents[2] == 0x44 && contents[3] == 0x46 && contents[4] == 0x2D
    }

    #[test]
    fn can_get_valid_issue_number_from_link() {
        let issue_url = format!("{SITE_URL}/issue1");

        let issue_number = get_issue_number_from_link(&issue_url);
        assert!(issue_number.is_some());
        assert!(issue_number.unwrap() == 1);
    }

    #[test]
    fn can_get_invalid_issue_number_from_link() {
        let issue_url = format!("{SITE_URL}/issue");
        let issue_number = get_issue_number_from_link(&issue_url);
        assert!(issue_number.is_none());

        let issue_url = format!("{SITE_URL}/issuenothing");
        let issue_number = get_issue_number_from_link(&issue_url);
        assert!(issue_number.is_none());

        let issue_url = format!("{SITE_URL}/issue1a");
        let issue_number = get_issue_number_from_link(&issue_url);
        assert!(issue_number.is_none());
    }

    #[test]
    fn can_find_last_issue_number() {
        let document = document::fetch_document().unwrap();
        let last_issue_number = find_last_issue_number(&document);
        assert!(last_issue_number.is_some());
    }

    #[test]
    fn can_download_issue() {
        let output_dir = temp_dir();
        assert!(download_issue(1, &output_dir).is_ok());

        let mut output_file = output_dir;
        output_file.push("1");
        output_file.set_extension("pdf");
        assert!(output_file.is_file());
        assert!(output_file.metadata().unwrap().size() > 1000);
        assert!(check_pdf_file_is_valid(&output_file));
    }

    #[test]
    fn can_download_issue_with_uppercase_hex() {
        let output_dir = temp_dir();
        assert!(download_issue(12, &output_dir).is_ok());

        let mut output_file = output_dir;
        output_file.push("12");
        output_file.set_extension("pdf");
        assert!(output_file.is_file());
        assert!(output_file.metadata().unwrap().size() > 1000);
        assert!(check_pdf_file_is_valid(&output_file));
    }
}
