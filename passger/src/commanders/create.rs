use jli::core as JLI;

pub fn create_commander(command_args: Result<JLI::CreateCommand, String>) {
    JLI::show_command_title("Create new passkey");

    let command = JLI::handle_command_err(command_args);

    match command {
        JLI::CreateCommand::Help => JLI::show_create_command_help(),
        JLI::CreateCommand::Create => {
            println!("Create new passkey.");
        }
    }
}
