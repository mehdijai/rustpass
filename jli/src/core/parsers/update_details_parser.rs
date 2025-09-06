use crate::core as JLI;

pub enum UpdateDetailsCommand {
    Help,
    Update {
        id: String,
        name: Option<String>,
        email: Option<String>,
    },
}

pub fn parse_update_details_command(
    options: Vec<(String, Option<String>)>,
) -> UpdateDetailsCommand {
    let possible_flags = vec!["--help", "-h", "-i", "--id", "-n", "-e"];

    let is_valid = JLI::validate_options(possible_flags, options.clone());

    match is_valid {
        Err(err) => com::print_error(err),
        Ok(()) => build_command_options(options),
    }
}

fn build_command_options(options: Vec<(String, Option<String>)>) -> UpdateDetailsCommand {
    let is_help = JLI::is_help_command(&options);

    if is_help {
        return UpdateDetailsCommand::Help;
    }

    let id = options
        .iter()
        .find(|(flag, _)| flag == "-i" || flag == "--id")
        .and_then(|(_, value)| value.clone());

    if id.is_none() {
        com::print_error(com::Error::InvalidInput("ID is required".to_string()));
    }

    let name = options
        .iter()
        .find(|(flag, _)| flag == "-n")
        .and_then(|(_, value)| value.clone());

    let email = options
        .iter()
        .find(|(flag, _)| flag == "-e")
        .and_then(|(_, value)| value.clone());

    if name.is_none() && email.is_none() {
        com::print_error(com::Error::InvalidInput(
            "Error: At least one of the following flags are required: -n, -e".to_string(),
        ));
    }

    UpdateDetailsCommand::Update {
        id: id.unwrap(),
        name,
        email,
    }
}
