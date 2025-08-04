/// Google command handler (default fallback)
/// Supports: [any search terms]
use crate::utils::bunnylol_command::BunnylolCommand;
use crate::utils::url_encoding::build_search_url;

pub struct GoogleCommand;

impl BunnylolCommand for GoogleCommand {
    const COMMAND: &'static str = ""; // Empty command means it's the default

    fn process_args(args: &str) -> String {
        build_search_url("https://google.com/search", "q", args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_command_simple() {
        assert_eq!(
            GoogleCommand::process_args("hello"),
            "https://google.com/search?q=hello"
        );
    }

    #[test]
    fn test_google_command_with_spaces() {
        assert_eq!(
            GoogleCommand::process_args("hello world"),
            "https://google.com/search?q=hello%20world"
        );
    }
}
