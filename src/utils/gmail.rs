pub fn construct_gmail_url(_query: &str) -> String {
    "https://mail.google.com".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_gmail_url() {
        let fake_query = "mail";
        assert_eq!(construct_gmail_url(fake_query), "https://mail.google.com");
    }
}
