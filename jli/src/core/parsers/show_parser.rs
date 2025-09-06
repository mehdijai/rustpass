use crate::core as JLI;

pub enum ShowCommand {
    Help,
    Show { id: String },
}

pub fn parse_show_command(options: Vec<(String, Option<String>)>) -> ShowCommand {
    let possible_flags = vec!["--help", "-h", "-i", "--id"];

    let is_valid = JLI::validate_options(possible_flags, options.clone());

    match is_valid {
        Err(err) => JLI::print_error(err),
        Ok(()) => build_command_options(options),
    }
}

fn build_command_options(options: Vec<(String, Option<String>)>) -> ShowCommand {
    let is_help = JLI::is_help_command(&options);

    if is_help {
        return ShowCommand::Help;
    }

    let id = options
        .iter()
        .find(|(flag, _)| flag == "-i" || flag == "--id")
        .and_then(|(_, value)| value.clone());

    if id.is_none() {
        JLI::print_error(JLI::Error::InvalidInput("ID is required".to_string()));
    }

    ShowCommand::Show { id: id.unwrap() }
}
