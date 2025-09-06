use jli::core as JLI;

pub fn delete_commander(command: JLI::DeleteCommand) {
    JLI::show_command_title("Show a passkey");

    match command {
        JLI::DeleteCommand::Help => JLI::show_delete_command_help(),
        JLI::DeleteCommand::Delete { id } => {
            println!("Id: {}", id);
        }
    }
}
