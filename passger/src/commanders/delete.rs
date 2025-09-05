use jli::core as JLI;

pub fn delete_commander(command_args: Result<JLI::DeleteCommand, String>) {
    JLI::show_command_title("Show a passkey");

    let command = JLI::handle_command_err(command_args);

    match command {
        JLI::DeleteCommand::Help => JLI::show_delete_command_help(),
        JLI::DeleteCommand::Delete { id } => {
            println!("Id: {}", id);
        }
    }
}
