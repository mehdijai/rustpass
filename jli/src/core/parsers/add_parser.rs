use crate::core::validate_options;

pub enum AddCommand {
    Help,
    Add { name: String, email: String },
}

pub fn parse_add_command(options: Vec<(String, Option<String>)>) -> Result<AddCommand, String> {
    let possible_flags = vec!["--help", "-h", "-n", "-e"];

    let is_valid = validate_options(possible_flags, options.clone());

    match is_valid {
        Err(err) => Err(err),
        Ok(()) => build_command_options(options),
    }
}

pub fn build_command_options(options: Vec<(String, Option<String>)>) -> Result<AddCommand, String> {
    let is_help = options
        .iter()
        .any(|(flag, _)| flag == "--help" || flag == "-h");

    if is_help {
        return Ok(AddCommand::Help);
    }

    let email = options
        .iter()
        .find(|(flag, _)| flag == "-e")
        .and_then(|(_, value)| value.clone());

    if email.is_none() {
        return Err("Error: Email is required".to_string());
    }

    let name = options
        .iter()
        .find(|(flag, _)| flag == "-n")
        .and_then(|(_, value)| value.clone());

    if name.is_none() {
        return Err("Error: Name is required".to_string());
    }

    Ok(AddCommand::Add {
        name: name.unwrap(),
        email: email.unwrap(),
    })
}
