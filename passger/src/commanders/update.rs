use jli::core as JLI;

pub fn update_commander(command_args: Result<JLI::UpdateCommand, String>) {
    JLI::show_command_title("Update a passkey");

    let command = JLI::handle_command_err(command_args);

    match command {
        JLI::UpdateCommand::Help => JLI::show_update_command_help(),
        JLI::UpdateCommand::Update { id } => {
            println!("Id: {}", id);
        }
    }
}
