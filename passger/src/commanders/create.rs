use jli::core as JLI;

pub fn create_commander(command: JLI::CreateCommand) {
    JLI::show_command_title("Create new passkey");

    match command {
        JLI::CreateCommand::Help => JLI::show_create_command_help(),
        JLI::CreateCommand::Create => {
            println!("Create new passkey.");
        }
    }
}
