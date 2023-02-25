use std::error::Error;

use select::document::Document;

pub const SITE_URL: &str = "https://www.digitalwhisper.co.il";

pub fn fetch_document() -> Result<Document, Box<dyn Error>> {
    let response = reqwest::blocking::get(SITE_URL)?.text()?;

    Ok(Document::from(response.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_fetch_document() {
        let document = fetch_document().unwrap();
        assert!(!document.nodes.is_empty());
    }
}
