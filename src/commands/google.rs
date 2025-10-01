/// Google command handler (default fallback)
/// Supports: [any search terms]
use crate::utils::bunnylol_command::BunnylolCommand;
use crate::utils::url_encoding::build_search_url;

pub struct GoogleCommand;

impl BunnylolCommand for GoogleCommand {
    const COMMAND: &'static str = "g";

    fn process_args(args: &str) -> String {
        let query = Self::get_command_args(args);
        build_search_url("https://google.com/search", "q", query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_command_simple() {
        assert_eq!(
            GoogleCommand::process_args("g hello"),
            "https://google.com/search?q=hello"
        );
    }

    #[test]
    fn test_google_command_with_spaces() {
        assert_eq!(
            GoogleCommand::process_args("g hello world"),
            "https://google.com/search?q=hello%20world"
        );
    }
}
