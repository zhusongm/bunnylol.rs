/// Confluence command handler
/// Supports: cf (simple redirect to confluence)
use crate::utils::bunnylol_command::BunnylolCommand;

pub struct ConfluenceCommand;

impl BunnylolCommand for ConfluenceCommand {
    const COMMAND: &'static str = "cf";

    fn process_args(_args: &str) -> String {
        "https://confluence.unity3d.com/".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gdocs_command() {
        assert_eq!(
            ConfluenceCommand::process_args("cf"),
            "https://confluence.unity3d.com/"
        );
    }

    #[test]
    fn test_gdoc_command_with_args() {
        // Gmail command ignores any additional arguments
        assert_eq!(
            ConfluenceCommand::process_args("cf some args"),
            "https://confluence.unity3d.com/"
        );
    }
}
