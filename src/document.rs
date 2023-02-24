use select::document::Document;

pub const SITE_URL: &str = "https://www.digitalwhisper.co.il";

pub fn fetch_document() -> Document {
    // let response = reqwest::blocking::get(SITE_URL)
    //     .expect("failed to load website")
    //     .text()
    //     .expect("failed to decode website content");
    // fs::write("output.txt", resp).unwrap();

    Document::from(include_str!("../output.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_fetch_document() {
        fetch_document();
    }
}
