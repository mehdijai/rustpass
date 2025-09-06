use jli::core as JLI;

pub fn update_commander(command: JLI::UpdateCommand) {
    JLI::show_command_title("Update a passkey");

    match command {
        JLI::UpdateCommand::Help => JLI::show_update_command_help(),
        JLI::UpdateCommand::Update { id } => {
            println!("Id: {}", id);
        }
    }
}
