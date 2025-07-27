/*
 * Copyright (c) Aaron Lichtman
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

pub fn construct_devbunny_url(query_string: &str) -> String {
    let cmd_part = if let Some(stripped) = query_string.strip_prefix("devbunny ") {
        stripped
    } else {
        query_string
    };

    format!(
        "http://localhost:8000/?cmd={}",
        utf8_percent_encode(cmd_part, NON_ALPHANUMERIC)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_devbunny_url() {
        let actual = construct_devbunny_url("devbunny test query");
        let expected = "http://localhost:8000/?cmd=test%20query";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_construct_devbunny_url_no_prefix() {
        let actual = construct_devbunny_url("test query");
        let expected = "http://localhost:8000/?cmd=test%20query";
        assert_eq!(actual, expected);
    }
}
