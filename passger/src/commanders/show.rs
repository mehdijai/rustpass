use jli::core as JLI;

pub fn show_commander(command: JLI::ShowCommand) {
    JLI::show_command_title("Show a passkey");

    match command {
        JLI::ShowCommand::Help => JLI::show_show_command_help(),
        JLI::ShowCommand::Show { id } => {
            println!("Id: {}", id);
        }
    }
}
