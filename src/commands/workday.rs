/// Workday command handler
/// Supports: wd (simple redirect to Workday)
use crate::utils::bunnylol_command::BunnylolCommand;

pub struct WorkdayCommand;

impl BunnylolCommand for WorkdayCommand {
    const COMMAND: &'static str = "wd";

    fn process_args(_args: &str) -> String {
        "https://www.myworkday.com/unitytech/d/home.htmld".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workday_command() {
        assert_eq!(
            WorkdayCommand::process_args("wd"),
            "https://www.myworkday.com/unitytech/d/home.htmld"
        );
    }

    #[test]
    fn test_workday_command_with_args() {
        // Workday command ignores any additional arguments
        assert_eq!(
            WorkdayCommand::process_args("wd some args"),
            "https://www.myworkday.com/unitytech/d/home.htmld"
        );
    }
}

