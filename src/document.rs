use select::document::Document;

pub const SITE_URL: &str = "https://www.digitalwhisper.co.il";

pub fn fetch_document() -> Document {
    let response = reqwest::blocking::get(SITE_URL)
        .expect("failed to load website")
        .text()
        .expect("failed to decode website content");

    Document::from(response.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_fetch_document() {
        let document = fetch_document();
        assert!(!document.nodes.is_empty());
    }
}
