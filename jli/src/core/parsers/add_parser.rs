use crate::core::{self as JLI};

pub enum AddCommand {
    Help,
    Add { name: String, email: String },
}

pub fn parse_add_command(options: Vec<(String, Option<String>)>) -> AddCommand {
    let possible_flags = vec!["--help", "-h", "-n", "-e"];

    let is_valid = JLI::validate_options(possible_flags, options.clone());

    match is_valid {
        Err(err) => com::print_error(err),
        Ok(()) => build_command_options(options),
    }
}

fn build_command_options(options: Vec<(String, Option<String>)>) -> AddCommand {
    let is_help = JLI::is_help_command(&options);

    if is_help {
        return AddCommand::Help;
    }

    let email = options
        .iter()
        .find(|(flag, _)| flag == "-e")
        .and_then(|(_, value)| value.clone());

    if email.is_none() {
        com::print_error(com::Error::InvalidInput("Email is required".to_string()));
    }

    let name = options
        .iter()
        .find(|(flag, _)| flag == "-n")
        .and_then(|(_, value)| value.clone());

    if name.is_none() {
        com::print_error(com::Error::InvalidInput("Name is required".to_string()));
    }

    AddCommand::Add {
        name: name.unwrap(),
        email: email.unwrap(),
    }
}
