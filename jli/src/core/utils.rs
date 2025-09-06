pub fn validate_options(
    possible_flags: Vec<&str>,
    options: Vec<(String, Option<String>)>,
) -> Result<(), com::Error> {
    for option in options {
        if !possible_flags.contains(&option.0.as_str()) {
            return Err(com::Error::InvalidFlag(option.0));
        }
    }

    Ok(())
}

pub fn handle_command_err<T>(command: Result<T, com::Error>) -> T {
    match command {
        Err(err) => com::print_error(err),
        Ok(command) => command,
    }
}

pub fn is_help_command(options: &Vec<(String, Option<String>)>) -> bool {
    options
        .iter()
        .any(|(flag, _)| flag == "--help" || flag == "-h")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_options() {
        let possible_flags = vec!["--help", "-h", "--version", "-v"];
        let options = vec![
            (String::from("--help"), None),
            (String::from("-h"), None),
            (String::from("--version"), None),
            (String::from("-v"), None),
        ];

        assert!(validate_options(possible_flags, options).is_ok());
    }

    #[test]
    fn test_validate_options_invalid_flag() {
        let possible_flags = vec!["--help", "-h", "--version", "-v"];
        let options = vec![
            (String::from("--help"), None),
            (String::from("-h"), None),
            (String::from("--version"), None),
            (String::from("-v"), None),
            (String::from("--invalid"), None),
        ];

        assert!(validate_options(possible_flags, options).is_err());
    }

    #[test]
    fn test_is_help_command() {
        let options = vec![
            (String::from("--help"), None),
            (String::from("-h"), None),
            (String::from("--version"), None),
            (String::from("-v"), None),
        ];

        assert!(is_help_command(&options));
    }

    #[test]
    fn test_is_help_command_no_help() {
        let options = vec![
            (String::from("--version"), None),
            (String::from("-v"), None),
        ];

        assert!(!is_help_command(&options));
    }
}
