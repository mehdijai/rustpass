use crate::core as JLI;

pub enum InitCommand {
    Help,
    Init,
}

pub fn parse_init_command(options: Vec<(String, Option<String>)>) -> Result<InitCommand, String> {
    let possible_flags = vec!["--help", "-h"];

    let is_valid = JLI::validate_options(possible_flags, options.clone());

    match is_valid {
        Err(err) => Err(err),
        Ok(()) => build_command_options(options),
    }
}

fn build_command_options(options: Vec<(String, Option<String>)>) -> Result<InitCommand, String> {
    let is_help = JLI::is_help_command(&options);

    if is_help {
        return Ok(InitCommand::Help);
    }

    Ok(InitCommand::Init)
}
