use crate::core as JLI;

pub enum CreateCommand {
    Help,
    Create,
}

pub fn parse_create_command(options: Vec<(String, Option<String>)>) -> CreateCommand {
    let possible_flags = vec!["--help", "-h"];

    let is_valid = JLI::validate_options(possible_flags, options.clone());

    match is_valid {
        Err(err) => com::print_error(err),
        Ok(()) => build_command_options(options),
    }
}

fn build_command_options(options: Vec<(String, Option<String>)>) -> CreateCommand {
    let is_help = JLI::is_help_command(&options);

    if is_help {
        return CreateCommand::Help;
    }

    CreateCommand::Create
}
