extern crate percent_encoding;

use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_rei_url(query: &str) -> String {
    if query == "rei" {
        "https://www.rei.com".to_string()
    } else {
        // Search format: "rei search term"
        let query_part = &query[4..]; // Remove "rei " prefix
        let encoded_query = utf8_percent_encode(query_part, FRAGMENT).to_string();
        format!("https://www.rei.com/search?q={}", encoded_query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_rei_url_basic() {
        let fake_query = "rei";
        assert_eq!(construct_rei_url(fake_query), "https://www.rei.com");
    }

    #[test]
    fn test_construct_rei_url_search() {
        let fake_query = "rei hiking boots";
        assert_eq!(
            construct_rei_url(fake_query),
            "https://www.rei.com/search?q=hiking%20boots"
        );
    }

    #[test]
    fn test_construct_rei_url_search_with_spaces() {
        let fake_query = "rei camping gear outdoor";
        assert_eq!(
            construct_rei_url(fake_query),
            "https://www.rei.com/search?q=camping%20gear%20outdoor"
        );
    }
}