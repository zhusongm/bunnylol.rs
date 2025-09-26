/// GDocs command handler
/// Supports: docs (simple redirect to docs)
use crate::utils::bunnylol_command::BunnylolCommand;

pub struct GDocsCommand;

impl BunnylolCommand for GDocsCommand {
    const COMMAND: &'static str = "docs";

    fn process_args(_args: &str) -> String {
        "https://docs.google.com".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gdocs_command() {
        assert_eq!(
            GDocsCommand::process_args("doc"),
            "https://docs.google.com"
        );
    }

    #[test]
    fn test_gdoc_command_with_args() {
        // Gmail command ignores any additional arguments
        assert_eq!(
            GDocsCommand::process_args("mail some args"),
            "https://docs.google.com"
        );
    }
}
