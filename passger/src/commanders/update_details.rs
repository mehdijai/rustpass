use jli::core as JLI;

pub fn update_details_commander(command: JLI::UpdateDetailsCommand) {
    JLI::show_command_title("Update passkey details (Name, and/or email)");

    match command {
        JLI::UpdateDetailsCommand::Help => JLI::show_update_details_command_help(),
        JLI::UpdateDetailsCommand::Update { id, name, email } => {
            println!("Id: {}", id);
            println!("Name: {:?}", name);
            println!("Email: {:?}", email);
        }
    }
}
