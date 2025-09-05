use std::process;

pub fn validate_options(
    possible_flags: Vec<&str>,
    options: Vec<(String, Option<String>)>,
) -> Result<(), String> {
    for option in options {
        if !possible_flags.contains(&option.0.as_str()) {
            return Err(format!("Invalid flag: {}", option.0));
        }
    }

    Ok(())
}

pub fn handle_command_err<T>(command: Result<T, String>) -> T {
    match command {
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(0);
        }
        Ok(command) => command,
    }
}

pub fn is_help_command(options: &Vec<(String, Option<String>)>) -> bool {
    options
        .iter()
        .any(|(flag, _)| flag == "--help" || flag == "-h")
}
