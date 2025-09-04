use std::env;

pub enum ArgCommands {
    Help,
    Version,
    None,
    NoCommand,
    Init(bool),
    Create(bool),
    Add(bool),
    List(bool),
    Show(bool),
    Delete(bool),
    Update(bool),
    UpdateMaster(bool),
}

pub fn read_args() -> ArgCommands {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        return ArgCommands::NoCommand;
    }
    match args[0].as_str() {
        "--help" => ArgCommands::Help,
        "--version" => ArgCommands::Version,
        "init" => ArgCommands::Init(false),
        "create" => ArgCommands::Create(false),
        "add" => ArgCommands::Add(false),
        "list" => ArgCommands::List(false),
        "show" => ArgCommands::Show(false),
        "delete" => ArgCommands::Delete(false),
        "update" => ArgCommands::Update(false),
        "update-master" => ArgCommands::UpdateMaster(false),
        _ => ArgCommands::None,
    }
}
