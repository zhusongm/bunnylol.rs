/// Google Groups command handler
/// Supports: group (simple redirect to Google Groups)
use crate::utils::bunnylol_command::BunnylolCommand;

pub struct GroupsCommand;

impl BunnylolCommand for GroupsCommand {
    const COMMAND: &'static str = "group";

    fn process_args(_args: &str) -> String {
        "https://groups.google.com".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_groups_command() {
        assert_eq!(
            GroupsCommand::process_args("group"),
            "https://groups.google.com"
        );
    }

    #[test]
    fn test_groups_command_with_args() {
        // Groups command ignores any additional arguments
        assert_eq!(
            GroupsCommand::process_args("group some args"),
            "https://groups.google.com"
        );
    }
}

