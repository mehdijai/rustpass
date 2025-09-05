use crate::core as JLI;

pub enum DeleteCommand {
    Help,
    Delete { id: String },
}

pub fn parse_delete_command(
    options: Vec<(String, Option<String>)>,
) -> Result<DeleteCommand, String> {
    let possible_flags = vec!["--help", "-h", "-i", "--id"];

    let is_valid = JLI::validate_options(possible_flags, options.clone());

    match is_valid {
        Err(err) => Err(err),
        Ok(()) => build_command_options(options),
    }
}

fn build_command_options(options: Vec<(String, Option<String>)>) -> Result<DeleteCommand, String> {
    let is_help = JLI::is_help_command(&options);

    if is_help {
        return Ok(DeleteCommand::Help);
    }

    let id = options
        .iter()
        .find(|(flag, _)| flag == "-i" || flag == "--id")
        .and_then(|(_, value)| value.clone());

    if id.is_none() {
        return Err("Error: ID is required".to_string());
    }

    Ok(DeleteCommand::Delete { id: id.unwrap() })
}
