use crate::core as JLI;
use std::env;

pub enum Commands {
    Help,
    Version,
    Add(Result<JLI::AddCommand, String>),
    Init(Result<JLI::InitCommand, String>),
    List(Result<JLI::ListCommand, String>),
    Create(Result<JLI::CreateCommand, String>),
    UpdateMaster(Result<JLI::UpdateMasterCommand, String>),
    Update(Result<JLI::UpdateCommand, String>),
    UpdateDetails(Result<JLI::UpdateDetailsCommand, String>),
    Show(Result<JLI::ShowCommand, String>),
    Delete(Result<JLI::DeleteCommand, String>),
}

pub fn parse_commands() -> Commands {
    let command = parse_args();
    match command {
        ArgCommands::Help => Commands::Help,
        ArgCommands::Version => Commands::Help,
        ArgCommands::NoCommand => Commands::Help,
        ArgCommands::Init(args) => Commands::Init(match_command(args, JLI::parse_init_command)),
        ArgCommands::Create(args) => {
            Commands::Create(match_command(args, JLI::parse_create_command))
        }
        ArgCommands::Add(args) => Commands::Add(match_command(args, JLI::parse_add_command)),
        ArgCommands::List(args) => Commands::List(match_command(args, JLI::parse_list_command)),
        ArgCommands::Show(args) => Commands::Show(match_command(args, JLI::parse_show_command)),
        ArgCommands::Delete(args) => {
            Commands::Delete(match_command(args, JLI::parse_delete_command))
        }
        ArgCommands::Update(args) => {
            Commands::Update(match_command(args, JLI::parse_update_command))
        }
        ArgCommands::UpdateDetails(args) => {
            Commands::UpdateDetails(match_command(args, JLI::parse_update_details_command))
        }
        ArgCommands::UpdateMaster(args) => {
            Commands::UpdateMaster(match_command(args, JLI::parse_update_master_command))
        }
    }
}

enum ArgCommands {
    Help,
    Version,
    NoCommand,
    Init(Vec<String>),
    Create(Vec<String>),
    Add(Vec<String>),
    List(Vec<String>),
    Show(Vec<String>),
    Delete(Vec<String>),
    Update(Vec<String>),
    UpdateDetails(Vec<String>),
    UpdateMaster(Vec<String>),
}

fn parse_args() -> ArgCommands {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        return ArgCommands::NoCommand;
    }
    match args[0].as_str() {
        "--help" => ArgCommands::Help,
        "-h" => ArgCommands::Help,
        "--version" => ArgCommands::Version,
        "-v" => ArgCommands::Version,
        "init" => ArgCommands::Init(args[1..].to_vec()),
        "create" => ArgCommands::Create(args[1..].to_vec()),
        "add" => ArgCommands::Add(args[1..].to_vec()),
        "list" => ArgCommands::List(args[1..].to_vec()),
        "show" => ArgCommands::Show(args[1..].to_vec()),
        "delete" => ArgCommands::Delete(args[1..].to_vec()),
        "update" => ArgCommands::Update(args[1..].to_vec()),
        "update-details" => ArgCommands::UpdateDetails(args[1..].to_vec()),
        "update-master" => ArgCommands::UpdateMaster(args[1..].to_vec()),
        _ => ArgCommands::Help,
    }
}

fn match_command<T>(args: Vec<String>, parser: fn(Vec<(String, Option<String>)>) -> T) -> T {
    let options = parse_options(&args);
    parser(options)
}

fn parse_options(args: &Vec<String>) -> Vec<(String, Option<String>)> {
    let mut params_options: Vec<(String, Option<String>)> = Vec::new();
    let mut i = 0;

    let mut options_args = args.clone();

    options_args.dedup();

    while i < options_args.len() {
        let arg = &options_args[i];

        if arg.starts_with("-") {
            let flag_name = arg.clone();

            let value = if i + 1 < options_args.len() && !options_args[i + 1].starts_with("-") {
                i += 1;
                Some(options_args[i].clone())
            } else {
                None
            };

            params_options.push((flag_name, value));
        }

        i += 1;
    }

    params_options
}
