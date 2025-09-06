use crate::core as JLI;

pub enum DeleteCommand {
    Help,
    Delete { id: String },
}

pub fn parse_delete_command(options: Vec<(String, Option<String>)>) -> DeleteCommand {
    let possible_flags = vec!["--help", "-h", "-i", "--id"];

    let is_valid = JLI::validate_options(possible_flags, options.clone());

    match is_valid {
        Err(err) => com::print_error(err),
        Ok(()) => build_command_options(options),
    }
}

fn build_command_options(options: Vec<(String, Option<String>)>) -> DeleteCommand {
    let is_help = JLI::is_help_command(&options);

    if is_help {
        return DeleteCommand::Help;
    }

    let id = options
        .iter()
        .find(|(flag, _)| flag == "-i" || flag == "--id")
        .and_then(|(_, value)| value.clone());

    if id.is_none() {
        com::print_error(com::Error::InvalidInput(
            "Error: ID is required".to_string(),
        ));
    }

    DeleteCommand::Delete { id: id.unwrap() }
}
