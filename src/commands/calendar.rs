/// Google Calendar command handler
/// Supports: cal (simple redirect to Google Calendar)
use crate::utils::bunnylol_command::BunnylolCommand;

pub struct CalendarCommand;

impl BunnylolCommand for CalendarCommand {
    const COMMAND: &'static str = "cal";

    fn process_args(_args: &str) -> String {
        "https://calendar.google.com".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_command() {
        assert_eq!(
            CalendarCommand::process_args("cal"),
            "https://calendar.google.com"
        );
    }

    #[test]
    fn test_calendar_command_with_args() {
        // Calendar command ignores any additional arguments
        assert_eq!(
            CalendarCommand::process_args("cal some args"),
            "https://calendar.google.com"
        );
    }
}
