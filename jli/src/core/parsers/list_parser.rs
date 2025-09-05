use crate::core as JLI;

pub enum ListCommand {
    Help,
    List,
}

pub fn parse_list_command(options: Vec<(String, Option<String>)>) -> Result<ListCommand, String> {
    let possible_flags = vec!["--help", "-h"];

    let is_valid = JLI::validate_options(possible_flags, options.clone());

    match is_valid {
        Err(err) => Err(err),
        Ok(()) => build_command_options(options),
    }
}

fn build_command_options(options: Vec<(String, Option<String>)>) -> Result<ListCommand, String> {
    let is_help = JLI::is_help_command(&options);

    if is_help {
        return Ok(ListCommand::Help);
    }

    Ok(ListCommand::List)
}
