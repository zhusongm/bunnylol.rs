/// GitHub command handler
/// Supports: gh, gh [user], gh [user/repo]
use crate::utils::bunnylol_command::BunnylolCommand;
use crate::utils::url_encoding::build_path_url;

pub struct GitHubCommand;

impl BunnylolCommand for GitHubCommand {
    const COMMAND: &'static str = "gh";

    fn process_args(args: &str) -> String {
        if args == Self::COMMAND {
            "https://github.com".to_string()
        } else {
            let query = Self::get_command_args(args);
            build_path_url("https://github.com", query)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_command_base() {
        assert_eq!(GitHubCommand::process_args("gh"), "https://github.com");
    }

    #[test]
    fn test_github_command_user() {
        assert_eq!(
            GitHubCommand::process_args("gh facebook"),
            "https://github.com/facebook"
        );
    }

    #[test]
    fn test_github_command_repo() {
        assert_eq!(
            GitHubCommand::process_args("gh facebook/react"),
            "https://github.com/facebook/react"
        );
    }
}
