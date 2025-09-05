use crate::core as JLI;

pub enum UpdateCommand {
    Help,
    Update { id: String },
}

pub fn parse_update_command(
    options: Vec<(String, Option<String>)>,
) -> Result<UpdateCommand, String> {
    let possible_flags = vec!["--help", "-h", "-i", "--id"];

    let is_valid = JLI::validate_options(possible_flags, options.clone());

    match is_valid {
        Err(err) => Err(err),
        Ok(()) => build_command_options(options),
    }
}

fn build_command_options(options: Vec<(String, Option<String>)>) -> Result<UpdateCommand, String> {
    let is_help = JLI::is_help_command(&options);

    if is_help {
        return Ok(UpdateCommand::Help);
    }

    let id = options
        .iter()
        .find(|(flag, _)| flag == "-i" || flag == "--id")
        .and_then(|(_, value)| value.clone());

    if id.is_none() {
        return Err("Error: Email is required".to_string());
    }

    Ok(UpdateCommand::Update { id: id.unwrap() })
}
