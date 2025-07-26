/*
 * Copyright (c) Aaron Lichtman
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

extern crate percent_encoding;

use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_reddit_url(query: &str) -> String {
    if query == "r" {
        "https://reddit.com".to_string()
    } else if query.starts_with("r/") {
        // Subreddit format: "r/programming"
        let subreddit = &query[2..]; // Remove "r/" prefix
        format!("https://reddit.com/r/{}", subreddit)
    } else {
        // Search format: "r search term"
        let query_part = &query[2..]; // Remove "r " prefix
        let encoded_query = utf8_percent_encode(query_part, FRAGMENT).to_string();
        format!("https://reddit.com/search?q={}", encoded_query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_reddit_url_basic() {
        let fake_query = "r";
        assert_eq!(construct_reddit_url(fake_query), "https://reddit.com");
    }

    #[test]
    fn test_construct_reddit_url_subreddit() {
        let fake_query = "r/programming";
        assert_eq!(
            construct_reddit_url(fake_query),
            "https://reddit.com/r/programming"
        );
    }

    #[test]
    fn test_construct_reddit_url_search() {
        let fake_query = "r programming help";
        assert_eq!(
            construct_reddit_url(fake_query),
            "https://reddit.com/search?q=programming%20help"
        );
    }
}
